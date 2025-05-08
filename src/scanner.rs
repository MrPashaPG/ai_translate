use std::{ffi::OsStr, fs, path::PathBuf, sync::LazyLock};

use crate::logger::Logger;
use crate::queue::FifoQueue;

static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new("Scanner"));

pub fn collect_subtitles_path(dir_path: &str, subtitles_queue: &mut FifoQueue<PathBuf>) {
    LOGGER.info(format!("Start scanning folder: '{}'\n", dir_path).as_str());

    let mut entries: Vec<_> = match fs::read_dir(dir_path) {
        Ok(entries) => entries.map(|res| res.unwrap()).collect(),
        Err(e) => {
            LOGGER.error(
                format!("Error reading directory contents of '{}': {}", dir_path, e).as_str(),
            );
            core::panic!();
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

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name() == Some(OsStr::new(env!("TRANSLATE_TARGET_DIR"))) {
                LOGGER.warning(format!("Skip this folder: {}\n", path.display()).as_str());
                continue;
            }
            collect_subtitles_path(path.to_str().unwrap(), subtitles_queue);
        } else if path.is_file() && path.extension().map_or(false, |ext| ext == "srt") {
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if !file_name.contains("_fa") {
                subtitles_queue.enqueue(path);
            }
        }
    }

    LOGGER.success(format!("End scanning folder: '{}'\n", dir_path).as_str());
}
