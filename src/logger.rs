#![allow(dead_code)]

// ANSI escape codes for colors and styles
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const DIM: &str = "\x1b[2m";
const BOLD: &str = "\x1b[1m";

// Emojis for log levels
const INFO_ICON: &str = "ℹ️ ";
const WARNING_ICON: &str = "⚠️ ";
const ERROR_ICON: &str = "❌";
const SUCCESS_ICON: &str = "✅";
const DEBUG_ICON: &str = "🐞";
const GEAR_ICON: &str = "⚙️ ";

pub struct Logger {
    module_name: &'static str,
}

impl Logger {
    pub fn new(module_name: &'static str) -> Self {
        Logger { module_name }
    }

    // Helper to format the prefix
    fn format_prefix(&self, color: &str, icon: &str, level: &str) -> String {
        format!(
            "{} [{}] {}({}){}",
            icon,  // Icon with color
            level, // Level text
            color,
            self.module_name,
            RESET // Module name with color
        )
    }

    pub fn info(&self, message: &str) {
        print!(
            "{}: {}{}{}\n",
            self.format_prefix(BLUE, INFO_ICON, "INFO"),
            BLUE,
            message,
            RESET,
        );
    }

    pub fn warning(&self, message: &str) {
        print!(
            "{}: {}{}{}\n",
            self.format_prefix(YELLOW, WARNING_ICON, "WARNING"),
            YELLOW,
            message,
            RESET,
        );
    }

    pub fn error(&self, message: &str) {
        print!(
            "{}: {}{}{}\n",
            self.format_prefix(RED, ERROR_ICON, "ERROR"),
            RED,
            message,
            RESET,
        );
    }

    pub fn success(&self, message: &str) {
        print!(
            "{}: {}{}{}\n",
            self.format_prefix(GREEN, SUCCESS_ICON, "SUCCESS"),
            GREEN,
            message,
            RESET,
        );
    }

    pub fn log(&self, message: &str) {
        // For general, unstyled messages, or specific formatting needs
        print!("{}", message);
    }

    pub fn debug(&self, message: &str) {
        if cfg!(debug_assertions) {
            print!(
                "{}: {}{}{}\n",
                self.format_prefix(DIM, DEBUG_ICON, "DEBUG"),
                DIM,
                message,
                RESET,
            );
        }
    }

    pub fn bold(&self, message: &str) {
        print!("{}{}{}\n", BOLD, message, RESET);
    }

    // It might be better to simplify and let the main color of the level dominate
    // or integrate boldness directly into the primary log functions if needed often.
    // For now, keeping them separate but consider if they are truly needed
    // or if `info`, `warning` etc. should have a `bold_message` parameter.

    pub fn bold_message_info(&self, message: &str) {
        print!(
            "{}: {}{}{}{}\n",
            self.format_prefix(BLUE, INFO_ICON, "INFO"),
            BLUE,
            BOLD,
            message,
            RESET
        );
    }

    pub fn bold_message_warning(&self, message: &str) {
        print!(
            "{}: {}{}{}{}\n",
            self.format_prefix(YELLOW, WARNING_ICON, "WARNING"),
            YELLOW,
            BOLD,
            message,
            RESET
        );
    }

    pub fn bold_message_error(&self, message: &str) {
        print!(
            "{}: {}{}{}{}\n",
            self.format_prefix(RED, ERROR_ICON, "ERROR"),
            RED,
            BOLD,
            message,
            RESET
        );
    }

    pub fn bold_message_success(&self, message: &str) {
        print!(
            "{}: {}{}{}{}\n",
            self.format_prefix(GREEN, SUCCESS_ICON, "SUCCESS"),
            GREEN,
            BOLD,
            message,
            RESET
        );
    }

    // A general purpose log with an icon, for processes or statuses
    pub fn process(&self, message: &str) {
        print!(
            "{}: {}{}{}\n",
            self.format_prefix(BLUE, GEAR_ICON, "PROCESS"), // Using blue and gear for general processes
            BLUE,
            message,
            RESET,
        );
    }
}

// ✅ 
// ℹ️ 
