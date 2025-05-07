use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    sync::LazyLock,
};

mod logger;

static LOGGER: LazyLock<logger::Logger> = LazyLock::new(|| logger::Logger::new("Main"));

fn main() {
    LOGGER.bold("This program is designed to translate English subtitles of educational courses into Persian using artificial intelligence.\n\n");
    LOGGER.bold_info(
        "Next, enter the path of the main course folder or any folder containing the subtitles you want to translate. 
The program will translate any subtitles found in that path. 
Additionally, if there are other subfolders within that directory, it will also translate the subtitles inside those folders.\n\n"
    );

    loop {
        LOGGER.bold("Please enter folder path to process: ");
        io::stdout().flush().unwrap();

        let mut terminal_input_buffer = String::new();
        match io::stdin().read_line(&mut terminal_input_buffer) {
            Ok(_) => {
                let terminal_input = terminal_input_buffer.trim();

                if terminal_input.eq_ignore_ascii_case("exit") || terminal_input.eq_ignore_ascii_case("quit") {
                    LOGGER.log("Exiting program...\n");
                    break;
                }
                if terminal_input.is_empty() {
                    LOGGER.bold_warning("Empty input. Please enter a valid folder path.\n");
                    continue;
                }

                let input_path = PathBuf::from(terminal_input);

                if !input_path.exists() {
                    LOGGER.bold_error(format!("Error: The specified root path '{}' does not exist.\n", input_path.display()).as_str());
                    continue;
                }
                if !input_path.is_dir() {
                    LOGGER.bold_error(format!("Error: The specified root path '{}' is not a directory.\n", input_path.display()).as_str());
                    continue;
                }
                
                LOGGER.info(format!("Start processing folder: '{}'\n", input_path.display()).as_str());
                // TODO
            }
            Err(error) => {
                LOGGER.bold_error(&format!("Error reading input: {}\n", error));
                break;
            }
        }

        LOGGER.bold("\n============================================\n");
        LOGGER.bold("Ready for next folder path...\n");
    }
}
