use arboard::{Clipboard};

use crate::{communication::{Message, Actions}, actions::Action};

pub struct ClipboardAction {
    clipboard: Clipboard
}

impl ClipboardAction {
    pub fn new(clipboard: Clipboard) -> Self {
        ClipboardAction { clipboard }
    }
}

impl Action for ClipboardAction {
    fn handle_message(&self, message: Message) {
        if message.method == Actions::CLIPBOARD {
            match message.body.get("type") {
                Some(clipboard_type) => {
                    println!("{}",clipboard_type);
                }
                _ => {

                }
            }
        }
    }
}
