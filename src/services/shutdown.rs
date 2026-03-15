use std::process::Command;

use crate::{communication::Message, services::Service};

pub struct ShutdownService;

impl Service for ShutdownService {
    fn handle_message(_message: Message) {
        Command::new("sh")
            .arg("-c")
            .arg("shutdown now")
            .output()
            .expect("failed to execute process");

        return ();
    }
}
