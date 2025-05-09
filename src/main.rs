use std::{
    io::{self, Write},
    path::PathBuf,
    sync::LazyLock,
};

mod logger;
mod parser;
mod queue;
mod scanner;
mod translator;
mod writer;

static LOGGER: LazyLock<logger::Logger> = LazyLock::new(|| logger::Logger::new("Main"));

fn main() {
    // let mut gemini_api = Option::<String>::None;
    let mut subtitles_queue = queue::FifoQueue::<PathBuf>::new();
    let max_lenght_line: usize;

    LOGGER.bold("This program is designed to translate English subtitles of educational courses into Persian using artificial intelligence.\n\n");

    max_lenght_line = terminal_max_lenght_line_input();
    LOGGER.bold_warning(
        format!(
            "Maximum length of each line in the subtitle file: {}\n",
            max_lenght_line
        )
        .as_str(),
    );

    loop {
        // if gemini_api.is_none() {
        //     match _terminal_api_key_input() {
        //         Ok(api_key) => {
        //             gemini_api = Some(api_key);
        //             LOGGER.bold("Gemini API key set successfully.\n");
        //         }
        //         Err(error_message) => {
        //             if error_message.eq("exit") {
        //                 break;
        //             } else if error_message.eq("continue") {
        //                 continue;
        //             }
        //         }
        //     }
        // }

        let dir_path = match terminal_path_input() {
            Ok(path) => path,
            Err(error_message) => {
                if error_message.eq("exit") {
                    break;
                } else if error_message.eq("continue") {
                    continue;
                }
                panic!()
            }
        };

        let _subtitles_path =
            scanner::collect_subtitles_path(dir_path.as_str(), &mut subtitles_queue);

        LOGGER.success(format!("Total subtitle files found: {}\n", subtitles_queue.len()).as_str());

        if subtitles_queue.is_empty() {
            LOGGER.bold_warning("No subtitle files found in the specified folder.\n");
            continue;
        }

        // for _ in 0..subtitles_queue.len() {
        for _ in 0..1 {
            let subtitle_path = subtitles_queue.dequeue().unwrap();
            LOGGER
                .info(format!("Processing subtitle file: {}\n", subtitle_path.display()).as_str());
            if scanner::subtitle_exists_in_target_dir(&subtitle_path) {
                LOGGER.warning(
                    format!(
                        "Subtitle file \"{}\" already exists in target directory: {}\n",
                        subtitle_path.file_name().unwrap().to_str().unwrap(),
                        env!("TRANSLATE_TARGET_DIR")
                    )
                    .as_str(),
                );
                continue;
            }
            let mut sub_deformated = parser::format_subtitle_file(subtitle_path.clone());
            let ai_string = parser::convert_vec_to_ai_string(sub_deformated[1].clone());
            LOGGER.info("Start translating...\n");
            let translated_content = translator::translate_subtitle(ai_string);
            LOGGER.success("End of translating\n");
            sub_deformated[1] = parser::convert_ai_string_to_vec(translated_content);
            let srt_content =
                parser::convert_formated_subtitle_to_srt_format(sub_deformated, max_lenght_line);
            LOGGER.success("End of converting to SRT format\n");
            writer::write_translated_and_copy_original(&subtitle_path, srt_content);
        }

        subtitles_queue.clear();
        LOGGER.bold("\n============================================\n");
        LOGGER.bold("Ready for next folder path...\n\n");
    }
}

fn terminal_path_input() -> Result<String, String> {
    LOGGER.bold_info(
        "Next, enter the path of the main course folder or any folder containing the subtitles you want to translate. 
The program will translate any subtitles found in that path. 
Additionally, if there are other subfolders within that directory, it will also translate the subtitles inside those folders.\n\n"
    );
    LOGGER.bold("Please enter folder path to process: ");
    io::stdout().flush().unwrap();

    let mut terminal_input_buffer = String::new();
    match io::stdin().read_line(&mut terminal_input_buffer) {
        Ok(_) => {
            let terminal_input = terminal_input_buffer.trim();

            if terminal_input.eq_ignore_ascii_case("exit")
                || terminal_input.eq_ignore_ascii_case("quit")
            {
                LOGGER.log("Exiting program...\n");
                return Err("exit".to_string());
            }
            if terminal_input.is_empty() {
                LOGGER.bold_warning("Empty input. Please enter a valid folder path.\n");
                return Err("continue".to_string());
            }

            let input_path = PathBuf::from(terminal_input);

            if !input_path.exists() {
                LOGGER.bold_error(
                    format!(
                        "Error: The specified root path '{}' does not exist.\n",
                        input_path.display()
                    )
                    .as_str(),
                );
                return Err("continue".to_string());
            }
            if !input_path.is_dir() {
                LOGGER.bold_error(
                    format!(
                        "Error: The specified root path '{}' is not a directory.\n",
                        input_path.display()
                    )
                    .as_str(),
                );
                return Err("continue".to_string());
            }

            return Ok(input_path.to_string_lossy().into_owned());
        }
        Err(error) => {
            LOGGER.bold_error(&format!("Error reading input: {}\n", error));
            return Err("exit".to_string());
        }
    }
}

fn _terminal_api_key_input() -> Result<String, String> {
    LOGGER.bold("Please enter your Gemini API key to use the translation service.\n");
    LOGGER.bold("You can get your Gemini API key from https://gemini.com/api.\n");
    LOGGER.bold("Please enter your Gemini API key: ");
    io::stdout().flush().unwrap();

    let mut api_key_buffer = String::new();
    match io::stdin().read_line(&mut api_key_buffer) {
        Ok(_) => {
            let api_key = api_key_buffer.trim();
            if api_key.is_empty() {
                LOGGER.bold_error("Empty input. Please enter a valid API key.\n");
                return Err("continue".to_string());
            }
            return Ok(api_key.to_string());
        }
        Err(error) => {
            LOGGER.bold_error(&format!("Error reading input: {}\n", error));
            return Err("exit".to_string());
        }
    }
}

fn terminal_max_lenght_line_input() -> usize {
    LOGGER.bold(
        "Please enter the maximum length of each line in the subtitle file (default is 55): ",
    );
    io::stdout().flush().unwrap();

    let mut max_length_buffer = String::new();
    loop {
        match io::stdin().read_line(&mut max_length_buffer) {
            Ok(_) => {
                let max_length = max_length_buffer.trim();
                if max_length.is_empty() {
                    return 55;
                }
                match max_length.parse::<usize>() {
                    Ok(length) => return length,
                    Err(_) => {
                        LOGGER.bold_error("Invalid input. Please enter a valid number.\n");
                        continue;
                    }
                }
            }
            Err(error) => {
                LOGGER.bold_error(&format!("Error reading input: {}\n", error));
                panic!();
            }
        }
    }
}
