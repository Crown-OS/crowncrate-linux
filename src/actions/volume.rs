use std::process::Command;

use crate::{communication::{Message, Actions}, actions::Action};

pub struct VolumeAction;

impl VolumeAction {
    pub fn new() -> Self {
        VolumeAction {}
    }
}

impl Action for VolumeAction {
    fn handle_message(&self, message: Message) {
        if message.method == Actions::VOLUME {
            if let Some(volume) = message.body.get("value") {
                Command::new("sh")
                    .arg("-c")
                    .arg(format!("pactl set-sink-volume 0 {}%", volume))
                    .output()
                    .expect("failed to execute process");
            }
        }
        return ();
    }
}
