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
                
                content_spliter[1]
                        .push(parts.next().unwrap_or_default().to_owned());
                
            }

            section_count += 1;
        });
    
    content_spliter
}

pub fn convert_to_ai_string(content: Vec<String>) -> String {
    let mut ai_string = String::new();

    for i in 0..content.len() {
        ai_string.push_str(
            format!("{}_{}\n", i, content[i]).as_str(),
        );
    }

    ai_string
}

fn read_file(file_path: PathBuf) -> String {
    let mut file_content = std::fs::File::open(file_path).expect("Failed to open file");

    let mut content = String::new();

    file_content
        .read_to_string(&mut content)
        .expect("Failed to read file");

    content
}
