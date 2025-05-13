use std::{
    io::{self, Write},
    path::PathBuf,
    sync::LazyLock,
};

use crate::logger::Logger;

static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new("CLI_Handler"));

pub fn get_user_path_input() -> Result<String, String> {
    LOGGER.bold("📂 Please enter the path to the main folder containing the subtitles you want to translate:");
    LOGGER.info("The program will scan this path and its subfolders for .srt files to translate.");
    LOGGER.log("Enter folder path: ");
    io::stdout().flush().unwrap();

    let mut terminal_input_buffer = String::new();
    match io::stdin().read_line(&mut terminal_input_buffer) {
        Ok(_) => {
            let terminal_input = terminal_input_buffer.trim();

            if terminal_input.eq_ignore_ascii_case("exit")
                || terminal_input.eq_ignore_ascii_case("quit")
            {
                LOGGER.warning("Exiting program as per user request...");
                return Err("exit".to_string());
            }
            if terminal_input.is_empty() {
                LOGGER.warning("Empty input. Please enter a valid folder path.");
                return Err("continue".to_string()); // User should retry
            }

            let input_path = PathBuf::from(terminal_input);

            if !input_path.exists() {
                LOGGER.error(
                    format!(
                        "The specified path '{}' does not exist.",
                        input_path.display()
                    )
                    .as_str(),
                );
                return Err("continue".to_string());
            }
            if !input_path.is_dir() {
                LOGGER.error(
                    format!(
                        "The specified path '{}' is not a directory.",
                        input_path.display()
                    )
                    .as_str(),
                );
                return Err("continue".to_string());
            }

            Ok(input_path.to_string_lossy().into_owned())
        }
        Err(error) => {
            LOGGER.error(&format!("Error reading input: {}", error));
            Err("exit".to_string()) // Critical error, suggest exit
        }
    }
}

pub fn get_api_key_input() -> Result<String, String> {
    LOGGER.bold("🔑 Please enter your Gemini API key to use the translation service:");
    LOGGER.info("You can obtain your API key from https://aistudio.google.com/app/apikey");
    LOGGER.log("Enter Gemini API Key: ");
    io::stdout().flush().unwrap();

    let mut api_key_buffer = String::new();
    match io::stdin().read_line(&mut api_key_buffer) {
        Ok(_) => {
            let api_key = api_key_buffer.trim();
            if api_key.is_empty() {
                LOGGER.warning("Empty input. Please enter a valid API key.");
                return Err("continue".to_string()); // User should retry
            }
            if api_key.eq_ignore_ascii_case("exit") || api_key.eq_ignore_ascii_case("quit") {
                LOGGER.warning("Exiting program as per user request...");
                return Err("exit".to_string());
            }
            Ok(api_key.to_string())
        }
        Err(error) => {
            LOGGER.error(&format!("Error reading input: {}", error));
            Err("exit".to_string()) // Critical error, suggest exit
        }
    }
}

pub fn get_max_line_length_input() -> usize {
    LOGGER.bold("📏 Enter the maximum length for each line in the output subtitle files (default: 55 characters):");
    LOGGER.log("Max line length (default 55): ");
    io::stdout().flush().unwrap();

    loop {
        let mut max_length_buffer = String::new();
        match io::stdin().read_line(&mut max_length_buffer) {
            Ok(_) => {
                let max_length_str = max_length_buffer.trim();
                if max_length_str.is_empty() {
                    LOGGER.info("Using default max line length (55).");
                    return 55;
                }
                if max_length_str.eq_ignore_ascii_case("exit")
                    || max_length_str.eq_ignore_ascii_case("quit")
                {
                    LOGGER.warning("Max line length input cancelled by user. Using default (55).");
                    return 55;
                }
                match max_length_str.parse::<usize>() {
                    Ok(length) => {
                        if length > 0 {
                            LOGGER.success(
                                format!("Maximum line length set to {}.", length).as_str(),
                            );
                            return length;
                        } else {
                            LOGGER.warning("Line length must be greater than zero. Please enter a valid number or leave empty for default.");
                            LOGGER.log("Max line length (default 55): ");
                            io::stdout().flush().unwrap();
                        }
                    }
                    Err(_) => {
                        LOGGER.warning("Invalid input. Please enter a valid integer or leave empty for default.");
                        LOGGER.log("Max line length (default 55): ");
                        io::stdout().flush().unwrap();
                    }
                }
            }
            Err(error) => {
                LOGGER.error(&format!(
                    "Error reading input: {}. Using default max line length (55).",
                    error
                ));
                return 55; // Default on critical read error
            }
        }
    }
}
