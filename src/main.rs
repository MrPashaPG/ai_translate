use std::{
    path::PathBuf,
    sync::LazyLock,
    io::{self, Write},
};

// Declare modules
mod utils;
mod logger;
mod parser;
mod queue;
mod scanner;
mod translator;
mod writer;
mod cli_handler;

// Global logger for main operations
static LOGGER: LazyLock<logger::Logger> = LazyLock::new(|| logger::Logger::new("Application"));

fn main() {
    print_welcome_message();

    let max_line_length = cli_handler::get_max_line_length_input();
    LOGGER.info(
        format!(
            "Maximum line length for output subtitle files will be: {} characters.",
            max_line_length
        )
        .as_str(),
    );
    LOGGER.log("\n");

    let mut gemini_api_key = match get_api_key_loop() {
        Some(key) => key,
        None => {
            LOGGER.bold("Program terminated by user.");
            return;
        }
    };

    let mut subtitles_queue = queue::FifoQueue::<PathBuf>::new();

    loop {
        match get_directory_path_loop() {
            Some(dir_path) => {
                process_directory(&dir_path, &mut subtitles_queue, &gemini_api_key, max_line_length);
            }
            None => break, // Exit main loop if user chose to exit during path input
        }

        LOGGER.bold("Process another folder? (y/n) or (c) to change API key:");
        io::stdout().flush().unwrap();
        let mut choice_buffer = String::new();
        if io::stdin().read_line(&mut choice_buffer).is_ok() {
            let choice = choice_buffer.trim().to_lowercase();
            if choice == "n" || choice == "exit" || choice == "quit" {
                LOGGER.info("User chose to exit.");
                break;
            } else if choice == "c" {
                 LOGGER.info("User chose to change API key.");
                 match get_api_key_loop() {
                    Some(key) => gemini_api_key = key,
                    None => { // User chose to exit during API key input
                        LOGGER.info("User chose to exit during API key change.");
                        break;
                    }
                }
            }
            // If 'y' or anything else, continue the loop for another folder
        } else {
            LOGGER.error("Failed to read user choice. Exiting.");
            break;
        }
        
        subtitles_queue.clear(); // Clear queue for the next batch
        LOGGER.log("\n============================================\n");
        LOGGER.info("Ready for the next folder path...\n");
    }
    LOGGER.bold("Program finished successfully. Goodbye!");
}

fn print_welcome_message() {
    LOGGER.bold("====================================================================");
    LOGGER.bold("🚀 Welcome to the AI Subtitle Translator!");
    LOGGER.info("This program is designed to translate English .SRT subtitles to Persian using the Gemini API.");
    LOGGER.bold("====================================================================\n");
}

fn get_api_key_loop() -> Option<String> {
    loop {
        match cli_handler::get_api_key_input() {
            Ok(api_key) => {
                LOGGER.success("Gemini API key set successfully.\n");
                return Some(api_key);
            }
            Err(err_msg) => {
                if err_msg == "exit" {
                    return None; // User chose to exit
                }
                // "continue" implies retry, loop continues
                LOGGER.warning("Please try again, or type 'exit' to quit.\n");
            }
        }
    }
}

fn get_directory_path_loop() -> Option<String> {
    loop {
        match cli_handler::get_user_path_input() {
            Ok(path) => return Some(path),
            Err(err_msg) => {
                if err_msg == "exit" {
                    return None; // User chose to exit
                }
                // "continue" implies retry, loop continues
                LOGGER.warning("Please try again, or type 'exit' to quit.\n");
            }
        }
    }
}

fn process_directory(
    dir_path: &str,
    subtitles_queue: &mut queue::FifoQueue<PathBuf>,
    gemini_api_key: &String,
    max_line_length: usize,
) {
    scanner::collect_subtitles_path(dir_path, subtitles_queue);

    if subtitles_queue.is_empty() {
        LOGGER.warning(format!("No subtitle files (.srt) found in the specified directory: '{}'.\n", dir_path).as_str());
        return;
    }

    LOGGER.success(format!("🔎 Total subtitle files found: {}\n", subtitles_queue.len()).as_str());

    let total_files = subtitles_queue.len();
    for i in 0..total_files {
        let subtitle_path = subtitles_queue.dequeue().unwrap(); 
        let subtitle_number = i + 1;

        let file_name_display = subtitle_path.file_name().unwrap_or_default().to_string_lossy();

        LOGGER.process(
            format!(
                "Processing file: {} (File {} of {})",
                file_name_display,
                subtitle_number,
                total_files
            )
            .as_str(),
        );
        
        // Log relative path for better context if deep in subfolders
        let relative_path = subtitle_path.strip_prefix(dir_path).unwrap_or(&subtitle_path);
        LOGGER.info(format!("Relative path: {}", relative_path.display()).as_str());

        let target_dir_env = env!("TRANSLATE_TARGET_DIR", "TRANSLATE_TARGET_DIR environment variable not set");

        if scanner::subtitle_exists_in_target_dir(&subtitle_path) {
            LOGGER.warning(
                format!(
                    "Subtitle file \"{}\" already exists in the target directory: {}, Skipping.",
                    file_name_display,
                    target_dir_env
                )
                .as_str(),
            );
            LOGGER.log("\n");
            continue;
        }

        match process_single_subtitle(&subtitle_path, gemini_api_key, max_line_length) {
            Ok(_) => LOGGER.success(format!("File '{}' processed and saved successfully.", file_name_display).as_str()),
            Err(e) => LOGGER.error(format!("Error processing file '{}': {}", file_name_display, e).as_str()),
        }
        LOGGER.log("\n"); 
    }
}

fn process_single_subtitle(
    subtitle_path: &PathBuf,
    gemini_api_key: &String,
    max_line_length: usize,
) -> Result<(), String> {
    LOGGER.info("Preparing and formatting subtitle content...");
    let mut sub_deformated = parser::format_subtitle_file(subtitle_path.clone());
    if sub_deformated[0].is_empty() && sub_deformated[1].is_empty() {
        // This can happen if read_file in parser fails and returns empty string, leading to empty splits.
        // The expect in read_file should ideally prevent this, but as a safeguard:
        return Err("Failed to parse subtitle file; content appears empty or corrupt.".to_string());
    }
    let ai_string = parser::convert_vec_to_ai_string(sub_deformated[1].clone());
    if ai_string.is_empty() && !sub_deformated[1].is_empty() {
         LOGGER.warning("Converted AI string is empty, but original subtitle parts were not. This might indicate an issue in `convert_vec_to_ai_string` or empty content lines.");
    }
     if ai_string.is_empty() && sub_deformated[1].iter().all(|s| s.trim().is_empty()) {
        LOGGER.warning("Subtitle file contains no translatable text content after parsing.");
        // Optionally, skip translation here if no actual text to translate
        // return Err("No translatable text content found in subtitle.".to_string());
    }


    LOGGER.info("⏳ Starting translation process with Gemini API...");
    match translator::translate_subtitle(ai_string, gemini_api_key.clone()) {
        Ok(translated_content) => {
            LOGGER.success("Translation completed successfully.");
            sub_deformated[1] = parser::convert_ai_string_to_vec(translated_content);
            
            LOGGER.info("Reconstructing subtitle file in SRT format...");
            let srt_content =
                parser::convert_formated_subtitle_to_srt_format(sub_deformated, max_line_length)?;
            
            writer::write_translated_and_copy_original(subtitle_path, srt_content);

            return Ok(());
        }
        Err(error) => {
            // More robust error checking from translator
            if error.starts_with("Error:") { 
                let err_msg = format!("Translation failed. API response: {}", error);
                // No need to log error here as translator module should have logged the specifics
                return Err(err_msg);
            }
            if error.is_empty() && !sub_deformated[1].iter().all(|s| s.trim().is_empty()){
                LOGGER.warning("Translation result is empty, but original content was not. This might indicate an API issue or full content filtering.");
                return Err("Translation resulted in empty content.".to_string()); // Or handle as appropriate
            }

            return Err(error);
        }
    };
}