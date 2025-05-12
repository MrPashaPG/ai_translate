use std::{env, fs, path::PathBuf, sync::LazyLock};

use crate::logger::Logger;

static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new("Writer"));

pub fn write_translated_and_copy_original(
    original_path: &PathBuf,
    srt_content: String,
) {
    let parent_dir = match original_path.parent() {
        Some(p) => p,
        None => {
            LOGGER.error(format!("Cannot access parent directory of '{}'. Write operation cancelled.", original_path.display()).as_str());
            return;
        }
    };
    let file_name = match original_path.file_name() {
        Some(f) => f,
        None => {
            LOGGER.error(format!("Cannot access file name of '{}'. Write operation cancelled.", original_path.display()).as_str());
            return;
        }
    };

    // Retrieve directory names from environment variables or use sensible defaults
    let target_dir_env_name = "TRANSLATE_TARGET_DIR";
    let original_sub_dir_env_name = "ORIGINAL_SUB_TARGET_DIR";

    let target_dir_name = env::var(target_dir_env_name)
        .unwrap_or_else(|_| {
            LOGGER.info(format!("Environment variable '{}' not set. Using default 'Translated_Subtitles'.", target_dir_env_name).as_str());
            "Translated_Subtitles".to_string()
        });
    let original_sub_dir_name = env::var(original_sub_dir_env_name)
        .unwrap_or_else(|_| {
            LOGGER.info(format!("Environment variable '{}' not set. Using default 'Original_Subtitles_Backup'.", original_sub_dir_env_name).as_str());
            "Original_Subtitles_Backup".to_string()
        });


    let target_dir_path = parent_dir.join(&target_dir_name);
    let original_sub_target_dir_path = parent_dir.join(&original_sub_dir_name);

    // Create target directory for translated files
    if let Err(e) = fs::create_dir_all(&target_dir_path) {
        LOGGER.error(format!("Error creating target directory for translations '{}': {}. Write operation cancelled.", target_dir_path.display(), e).as_str());
        return;
    }
    
    // Create backup directory for original files
    if let Err(e) = fs::create_dir_all(&original_sub_target_dir_path) {
        LOGGER.error(format!("Error creating backup directory for original subtitles '{}': {}. Original file will not be copied.", original_sub_target_dir_path.display(), e).as_str());
        // Continue to write translated file even if backup directory creation fails, but log the error.
    }

    // Write the translated SRT file
    let target_file_path = target_dir_path.join(file_name);
    match fs::write(&target_file_path, srt_content) {
        Ok(_) => {
            LOGGER.success(
                format!(
                    "Translated subtitle file saved successfully to: '{}'",
                    target_file_path.display()
                )
                .as_str(),
            );
        }
        Err(e) => {
            LOGGER.error(
                format!(
                    "Error writing translated file to '{}': {}",
                    target_file_path.display(),
                    e
                )
                .as_str(),
            );
            return; // If writing translated file fails, abort further operations for this file.
        }
    }

    // Copy the original subtitle file to the backup directory, only if its creation was successful or already existed.
    if original_sub_target_dir_path.exists() || fs::create_dir_all(&original_sub_target_dir_path).is_ok() {
        let original_backup_file_path = original_sub_target_dir_path.join(file_name);
        match fs::copy(original_path, &original_backup_file_path) {
            Ok(_) => {
                LOGGER.success(
                    format!(
                        "Original subtitle file successfully copied to backup: '{}'",
                        original_backup_file_path.display()
                    )
                    .as_str(),
                );
            }
            Err(e) => {
                LOGGER.warning( 
                    format!(
                        "Error copying original subtitle file to backup '{}': {}. The original file remains at its initial location.",
                        original_backup_file_path.display(),
                        e
                    )
                    .as_str(),
                );
            }
        }
    } else {
         LOGGER.warning(format!("Skipped copying original subtitle to backup because directory '{}' could not be ensured.", original_sub_target_dir_path.display()).as_str());
    }
}