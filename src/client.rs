use std::io;
use std::net::SocketAddr;

use futures::Future;
use futures::{Poll, Sink, Stream, StartSend, Async};
use tokio_core::reactor::Handle;
use tokio_io::{AsyncWrite, AsyncRead};
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_core::net::TcpStream;
use tokio_proto::TcpClient;
use tokio_proto::multiplex::{ClientProto, ClientService, NewRequestIdSource, RequestIdSource};
use tokio_service::Service;
use bytes::BytesMut;

use package::Package;
use codec::PackageCodec;

use uuid::Uuid;

/// `tokio_service::Service` implementation of the client.
pub struct EventStoreClient {
    inner: ClientService<TcpStream, PackageProto>,
}

impl EventStoreClient {
    /// Connect to an EventStore database listening at given `addr` using the given
    /// `tokio::reactor::Core`s `handle`.
    /// Returns a future representing the client which can be used to send and receive `Package`
    /// values.
    pub fn connect(addr: &SocketAddr, handle: &Handle) -> Box<Future<Item = Self, Error = io::Error>> {
        let ret = TcpClient::new(PackageProto)
            .connect(addr, handle)
            .map(|client_service| {
                EventStoreClient { inner: client_service }
            });

        Box::new(ret)
    }
}

impl Service for EventStoreClient {
    type Request = Package;
    type Response = Package;
    type Error = io::Error;
    type Future = Box<Future<Item = Package, Error = io::Error>>;

    fn call(&self, req: Package) -> Self::Future {
        self.inner.call(req).boxed()
    }
}

/// Simple heartbeats middleware
struct Heartbeats<T> {
    /// The upstream transport
    upstream: T,
}

// https://github.com/tokio-rs/tokio-line/blob/e2cf5758198cf56d87a064da34ffc7ad8ff8b13c/simple/examples/ping_pong.rs#L39

/// Implement `Stream` for our transport heartbeats middleware
impl<T> Stream for Heartbeats<T>
    where T: Stream<Item = (Uuid, Package), Error = io::Error>,
          T: Sink<SinkItem = (Uuid, Package), SinkError = io::Error>,
{
    type Item = (Uuid, Package);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {

        println!("Heartbeats handler!");
        use raw::RawMessage::{HeartbeatRequest, HeartbeatResponse};
        let x = try_ready!(self.upstream.poll());
        if let Some((_, Package { ref authentication, ref correlation_id, message: HeartbeatRequest })) = x {
            println!("Here is an HeartbeatRequest!");

            let heartbeat = Package {
                authentication: authentication.clone(), // FIXME don't clone
                correlation_id: *correlation_id,
                message: HeartbeatResponse,
            };
            let res = self.start_send((*correlation_id, heartbeat))?;

            assert!(res.is_ready()); // FIXME

            self.poll_complete()?;

            // FIXME what did I return here ?
            // did I break here and return NotReady ?
        }
        // FIXME I always return the message here, even if this is an handled one
        return Ok(Async::Ready(x))
    }
}

// FIXME I don't know if that the right way
impl<T> Sink for Heartbeats<T>
    where T: Sink<SinkItem = (Uuid, Package), SinkError = io::Error> {

    type SinkItem = (Uuid, Package);
    type SinkError = io::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.upstream.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.upstream.poll_complete()
    }
}

pub struct Separator;

impl Decoder for Separator {
    type Item = (Uuid, Package);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        PackageCodec.decode(buf).map(|x| x.map(|x| (x.correlation_id, x)))
    }
}

impl Encoder for Separator {
    type Item = (Uuid, Package);
    type Error = io::Error;

    fn encode(&mut self, (id, pkg): Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        assert_eq!(id, pkg.correlation_id);
        PackageCodec.encode(pkg, buf)
    }
}

impl RequestIdSource<Uuid, Package> for Separator {
    fn next(&mut self, pkg: &Package) -> Uuid {
        pkg.correlation_id
    }
}

impl NewRequestIdSource<Uuid, Package> for Uuid {
    type RequestIdSource = Separator;

    fn requestid_source() -> Self::RequestIdSource {
        Separator
    }
}

struct PackageProto;

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for PackageProto {
    type Request = Package;
    type Response = Package;
    type RequestId = Uuid;

    type Transport = Heartbeats<Framed<T, Separator>>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(Heartbeats {
            upstream: io.framed(Separator)
        })
    }
}
