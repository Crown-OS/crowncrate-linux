mod communication;
mod config;
mod discovery;
mod logging;
mod actions;

use std::{thread};

use arboard::Clipboard;
use communication::{Server};

use crate::{actions::{action_manager::ActionManager, clipboard::ClipboardAction, shutdown::ShutdownAction, volume::VolumeAction}, logging::{ConsoleLogger, FileLogger}};

fn main() {
    let mut action_manager = ActionManager::new();
    let logger = ConsoleLogger::new();

    let clipboard = Clipboard::new().unwrap();

    let clipboard_action = ClipboardAction::new(clipboard);

    action_manager.subscribe(communication::Actions::SHUTDOWN, Box::new(ShutdownAction::new()));
    action_manager.subscribe(communication::Actions::VOLUME, Box::new(VolumeAction::new()));
    action_manager.subscribe(communication::Actions::CLIPBOARD, Box::new(clipboard_action));

    let mut server = Server::create(action_manager);
    server.listen(5252).ok();
}
