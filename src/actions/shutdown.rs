use std::process::Command;

use crate::{communication::Message, actions::Action};

pub struct ShutdownAction;

impl ShutdownAction {
    pub fn new() -> Self {
        ShutdownAction {}
    }
}

impl Action for ShutdownAction {
    fn handle_message(&self, _message: Message) {
        Command::new("sh")
            .arg("-c")
            .arg("shutdown now")
            .output()
            .expect("failed to execute process");

        return ();
    }
}
