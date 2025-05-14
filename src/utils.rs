use regex::Regex;

pub fn formated_to_fa_srt_name(name: &str) -> String {
    let re = Regex::new(r"\.(?:en\.)?srt$").unwrap();
    re.replace(name, ".fa.srt").to_string()
}

pub fn formated_en_srt_name(name: &str) -> String {
    let re = Regex::new(r"\.(?:en\.)?srt$").unwrap();
    re.replace(name, ".en.srt").to_string()
}
