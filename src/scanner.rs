use std::{env, ffi::OsStr, fs, path::PathBuf, sync::LazyLock}; // Added env for robust env var handling

use crate::logger::Logger;
use crate::queue::FifoQueue;

static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new("Scanner"));

pub fn collect_subtitles_path(dir_path: &str, subtitles_queue: &mut FifoQueue<PathBuf>) {
    LOGGER.info(format!("🔍 Scanning folder: '{}'", dir_path).as_str());

    let entries_result = fs::read_dir(dir_path);
    let mut entries: Vec<_> = match entries_result {
        Ok(read_dir) => read_dir
            .filter_map(|res| {
                if res.is_err() {
                    LOGGER.warning(
                        format!("Error reading an entry in '{}', skipping entry.", dir_path)
                            .as_str(),
                    );
                }
                res.ok() // Convert Result<DirEntry, Error> to Option<DirEntry>
            })
            .collect(),
        Err(e) => {
            LOGGER.error(
                format!(
                    "Error reading directory contents of '{}': {}. Skipping this directory.",
                    dir_path, e
                )
                .as_str(),
            );
            return;
        }
    };

    entries.sort_by(|a, b| {
        let a_is_file = a.path().is_file();
        let b_is_file = b.path().is_file();
        match (a_is_file, b_is_file) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.path().cmp(&b.path()),
        }
    });

    // Get target directory names from env variables or use defaults
    let target_dir_env_name = "TRANSLATE_TARGET_DIR";
    let original_sub_dir_env_name = "ORIGINAL_SUB_TARGET_DIR";

    let target_dir_name = env::var(target_dir_env_name).unwrap_or_else(|_| {
        LOGGER.warning(
            format!(
                "Environment variable '{}' not set. Using default 'Translated_Subtitles'.",
                target_dir_env_name
            )
            .as_str(),
        );
        "Translated_Subtitles".to_string()
    });
    let original_sub_dir_name = env::var(original_sub_dir_env_name).unwrap_or_else(|_| {
        LOGGER.warning(
            format!(
                "Environment variable '{}' not set. Using default 'Original_Subtitles_Backup'.",
                original_sub_dir_env_name
            )
            .as_str(),
        );
        "Original_Subtitles_Backup".to_string()
    });

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name() == Some(OsStr::new(target_dir_name.as_str()))
                || path.file_name() == Some(OsStr::new(original_sub_dir_name.as_str()))
            {
                LOGGER.info(
                    format!("➡️  Skipping scan of system folder: '{}'", path.display()).as_str(),
                );
                continue;
            }

            if let Some(path_str) = path.to_str() {
                collect_subtitles_path(path_str, subtitles_queue);
            } else {
                LOGGER.warning(
                    format!(
                        "Path '{}' contains non-UTF8 characters, skipping.",
                        path.display()
                    )
                    .as_str(),
                );
            }
        } else if path.is_file()
            && path
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("srt"))
        {
            let file_name = path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();

            if !file_name.contains("_fa") && !file_name.ends_with("_fa.srt") {
                // More specific check for "_fa"
                LOGGER.debug(format!("Subtitle file found: {}", path.display()).as_str());
                subtitles_queue.enqueue(path);
            } else {
                LOGGER.info(
                    format!(
                        "File '{}' appears to be already translated (contains '_fa'). Skipping.",
                        file_name
                    )
                    .as_str(),
                );
            }
        }
    }
    // LOGGER.success(format!("Finished scanning folder: '{}'", dir_path).as_str()); // Optional: can be verbose
}

pub fn subtitle_exists_in_target_dir(original_path: &PathBuf) -> bool {
    let parent_dir = match original_path.parent() {
        Some(dir) => dir,
        None => {
            LOGGER.warning(format!("Could not determine parent directory for '{}'. Assuming file does not exist in target.", original_path.display()).as_str());
            return false;
        }
    };

    let target_dir_env_name = "TRANSLATE_TARGET_DIR";
    let target_dir_name = env::var(target_dir_env_name).unwrap_or_else(|_| {
        // This warning will appear for every file if env var is not set. Consider setting a static default or logging once.
        // LOGGER.warning(format!("Environment variable '{}' not set for target check. Using default 'Translated_Subtitles'.", target_dir_env_name).as_str());
        "Translated_Subtitles".to_string()
    });

    let fa_dir = parent_dir.join(target_dir_name);

    if !fa_dir.is_dir() {
        LOGGER.debug(
            format!(
                "Target directory '{}' does not exist yet (for checking file existence).",
                fa_dir.display()
            )
            .as_str(),
        );
        return false;
    }

    let file_name = match original_path.file_name() {
        Some(name) => name,
        None => {
            LOGGER.warning(format!("Could not determine file name for '{}'. Assuming file does not exist in target.", original_path.display()).as_str());
            return false;
        }
    };
    let fa_file_path = fa_dir.join(file_name);

    let exists = fa_file_path.is_file();
    if exists {
        LOGGER.debug(
            format!(
                "File '{}' found in target directory.",
                fa_file_path.display()
            )
            .as_str(),
        );
    }
    exists
}
