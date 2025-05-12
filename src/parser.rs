use std::{io::Read, path::PathBuf};

pub fn format_subtitle_file(file_path: PathBuf) -> [Vec<String>; 2] {
    let file_content = read_file(file_path);
    let mut content_spliter = [Vec::<String>::new(), Vec::<String>::new()];
    let mut section_count = 0;

    file_content
        .split("\r\n\r\n")
        .into_iter()
        .for_each(|section| {
            if section.len() > 0 {
                let mut parts = section.splitn(3, "\r\n");

                content_spliter[0].push("".to_string());

                for _ in 0..2 {
                    content_spliter[0][section_count]
                        .push_str((parts.next().unwrap_or_default().to_owned() + "\r\n").as_str());
                }

                content_spliter[1].push(parts.next().unwrap_or_default().to_owned());
            }

            section_count += 1;
        });

    content_spliter
}

pub fn convert_vec_to_ai_string(content: Vec<String>) -> String {
    let mut ai_string = String::new();

    for i in 0..content.len() {
        ai_string.push_str(format!("{}_{}\n", i, content[i]).as_str());
    }

    ai_string
}

pub fn convert_ai_string_to_vec(content: String) -> Vec<String> {
    let mut ai_string = Vec::new();

    content.split("\n").into_iter().for_each(|line| {
        if line.len() > 0 {
            let mut parts = line.splitn(2, "_");
            let _index = parts.next().unwrap_or_default();
            ai_string.push(parts.next().unwrap_or_default().to_owned());
        }
    });

    ai_string
}

pub fn convert_formated_subtitle_to_srt_format(
    formated_sub: [Vec<String>; 2],
    max_width: usize,
) -> String {
    let mut srt_content = String::new();

    for i in 0..formated_sub[0].len() {
        let warped_sub = wrap_with_markers(&formated_sub[1][i], max_width);
        srt_content.push_str(formated_sub[0][i].as_str());
        srt_content.push_str(warped_sub.as_str());
        srt_content.push_str("\r\n\r\n");
    }

    srt_content
}

fn read_file(file_path: PathBuf) -> String {
    let mut file_content = std::fs::File::open(file_path).expect("Failed to open file");

    let mut content = String::new();

    file_content
        .read_to_string(&mut content)
        .expect("Failed to read file");

    content
}

pub fn wrap_with_markers(text: &String, max_width: usize) -> String {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let tentative = if current.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current, word)
        };

        if tentative.chars().count() > max_width && max_width > 0 {
            lines.push(current);
            current = word.to_string();
        } else {
            current = tentative;
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
        .into_iter()
        .map(|line| format!("\u{202b}{}\u{202c}", line))
        .collect::<Vec<_>>()
        .join("\n")
}
