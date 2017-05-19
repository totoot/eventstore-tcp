use std::borrow::Cow;
use event_number::EventNumber;
use super::Message;
use raw::client_messages;

#[derive(Debug)]
pub struct ReadEvent<'a> {
    pub event_stream_id: Cow<'a, str>,
    pub event_number: EventNumber,
    pub resolve_link_tos: bool,
    pub require_master: bool,
}

impl<'a> From<ReadEvent<'a>> for Message<'a> {
    fn from(re: ReadEvent<'a>) -> Self {
        Message::ReadEvent(client_messages::ReadEvent {
            event_stream_id: re.event_stream_id,
            event_number: re.event_number.into(),
            resolve_link_tos: re.resolve_link_tos,
            require_master: re.require_master
        })
    }
}
