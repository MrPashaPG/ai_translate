use std::{env, fs, path::PathBuf, sync::LazyLock};

use crate::logger::Logger;

static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new("Writer"));

pub fn write_translated_and_copy_original(
    original_path: &PathBuf,
    srt_content: String,
) {
    let parent_dir = original_path
        .parent().unwrap();
    let file_name = original_path
        .file_name().unwrap();

    let target_dir = parent_dir.join(env!("TRANSLATE_TARGET_DIR"));
    let orginal_sub_target = parent_dir.join(env!("ORIGINAL_SUB_TARGET_DIR"));
    fs::create_dir_all(&target_dir).unwrap();
    fs::create_dir_all(&orginal_sub_target).unwrap();

    let target_file_path = target_dir.join(file_name);
    fs::write(&target_file_path, srt_content).unwrap();

    LOGGER
        .success(format!(
            "Translated subtitle file saved to: {}\n",
            target_file_path.display()
        )
        .as_str());

    let orginal_file_path = orginal_sub_target.join(file_name);
    fs::copy(&original_path, &orginal_file_path).unwrap();

    LOGGER
        .success(format!(
            "Original subtitle file copied to: {}\n",
            orginal_file_path.display()
        )
        .as_str());
}