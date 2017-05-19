use std::convert::TryFrom;
use raw::client_messages;
use quick_protobuf::BytesReader;

mod read_event;
pub use self::read_event::ReadEvent;

/// Enumeration of converted messages.
#[derive(Debug)]
pub enum Message<'a> {
    /// Requests heartbeat from the other side. Unsure if clients or server sends these.
    HeartbeatRequest,
    /// Response to a heartbeat request.
    HeartbeatResponse,

    /// Ping request, similar to heartbeat.
    Ping,
    /// Ping response.
    Pong,

    /// Request to read a single event from a stream
    ReadEvent(client_messages::ReadEvent<'a>),
}

impl<'a> TryFrom<&'a [u8]> for Message<'a> {
    type Error = (); // temporary of course !

    fn try_from(buffer: &'a [u8]) -> Result<Message<'a>, Self::Error> {
        if buffer.is_empty() {
            return Err( () );
        }

        let discriminator = buffer[0];
        let buffer = &buffer[1..]; // advance the buffer by one

        // TODO move this elsewhere (a Discriminator type or whatever name it have)
        Ok(match discriminator {
            0x01 => Message::HeartbeatRequest,
            0x02 => Message::HeartbeatResponse,
            0x03 => Message::Ping,
            0x04 => Message::Pong,

            0x82 => {
                let mut reader = BytesReader::from_bytes(buffer);
                let read_event = client_messages::ReadEvent::from_reader(&mut reader, buffer).or(Err(()))?; // FIXME don't ignore the error
                Message::ReadEvent(read_event)
            }

            _ => return Err( () )
        })
    }
}
