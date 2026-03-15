use std::process::Command;

use crate::{communication::Message, services::Service};

pub struct VolumeService;

impl Service for VolumeService {
    fn handle_message(message: Message) {
        let volume = message.body.get("value").unwrap();

        Command::new("sh")
            .arg("-c")
            .arg(format!("pactl set-sink-volume 0 {}%", volume))
            .output()
            .expect("failed to execute process");

        return ();
    }
}
