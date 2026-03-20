use crate::communication::{Message};

pub trait Action {
    fn handle_message(&self, message: Message);
}

