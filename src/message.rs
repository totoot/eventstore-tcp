use std::convert::TryFrom;
use raw::client_messages::*;
use quick_protobuf::BytesReader;

#[derive(Debug)]
pub enum Message<'a> {
    HeartbeatRequest,
    HeartbeatResponse,
    Ping,
    Pong,

    WriteEvents {
        inner: WriteEvents<'a>
    },
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
                Message::WriteEvents {
                    inner: WriteEvents::from_reader(&mut reader, buffer).map_err(|_| ())? // FIXME don't ignore the error
                }
            }

            _ => return Err( () )
        })
    }
}
