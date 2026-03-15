use crate::communication::Message;

pub trait Service {
    fn handle_message(message: Message);
}
