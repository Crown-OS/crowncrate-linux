use std::fs::{File};
use std::io::Write;

trait Logger {
    fn log(&mut self, message: String);
    fn warn(message: String);
    fn debug(message: String);
    fn error(message: String);
}

pub struct ConsoleLogger;

impl ConsoleLogger {
    pub fn new() -> Self {
        ConsoleLogger {}
    }
}

impl Logger for ConsoleLogger {
    fn log(&mut self, message: String) {
        println!("[LOG]: {}", message);
    }

    fn debug(message: String) {
        println!("[DEBUG]: {}", message);
    }

    fn warn(message: String) {
        println!("[WARNING]: {}", message);
    }

    fn error(message: String) {
        println!("[ERROR]: {}", message);
    }
}

pub struct FileLogger {
    file: File
}

impl FileLogger {
    pub fn new(file: File) -> Self {
        FileLogger { file }
    }
}

impl Logger for FileLogger {
    fn log(&mut self, message: String) {
        writeln!(&mut self.file, "[LOG]: {}", message);
    }

    fn debug(message: String) {
        println!("[DEBUG]: {}", message);
    }

    fn warn(message: String) {
        println!("[WARNING]: {}", message);
    }

    fn error(message: String) {
        println!("[ERROR]: {}", message);
    }
}
