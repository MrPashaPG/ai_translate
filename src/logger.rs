#![allow(dead_code)]

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const DIM: &str = "\x1b[2m";
const BOLD: &str = "\x1b[1m";

pub struct Logger{
    state: &'static str,
}

impl Logger{
    pub fn new(state: &'static str) -> Self {
        Logger { state }
    }

    pub fn info(&self, message: &str) {
        print!("{}[INFO][{}]{} {}", BLUE, self.state, RESET, message);
    }

    pub fn warning(&self, message: &str) {
        print!("{}[WARNING][{}]{} {}", YELLOW, self.state, RESET, message);
    }

    pub fn error(&self, message: &str) {
        print!("{}[ERROR][{}]{} {}", RED, self.state, RESET, message);
    }

    pub fn success(&self, message: &str) {
        print!("{}[SUCCESS][{}]{} {}", GREEN, self.state, RESET, message);
    }

    pub fn log(&self, message: &str) {
        print!("{}", message);
    }

    pub fn debug(&self, message: &str) {
        if cfg!(debug_assertions) {
            print!("{}[DEBUG][{}]{} {}", DIM, self.state, RESET, message);
        }
    }

    pub fn bold(&self, message: &str) {
        print!("{}{}{}", BOLD, message, RESET);
    }

    pub fn bold_info(&self, message: &str) {
        print!("{}[INFO][{}]{} {}{}{}", BLUE, self.state, RESET, BOLD, message, RESET);
    }

    pub fn bold_warning(&self, message: &str) {
        print!("{}[WARNING][{}]{} {}{}{}", YELLOW, self.state, RESET, BOLD, message, RESET);
    }

    pub fn bold_error(&self, message: &str) {
        print!("{}[ERROR][{}]{} {}{}{}", RED, self.state, RESET, BOLD, message, RESET);
    }

    pub fn bold_success(&self, message: &str) {
        print!("{}[SUCCESS][{}]{} {}{}{}", GREEN, self.state, RESET, BOLD, message, RESET);
    }
}
