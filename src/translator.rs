#![allow(non_snake_case)]
#![allow(dead_code)]

use regex::Regex;
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use crate::logger::Logger;

static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new("Translator"));

pub fn translate_subtitle(subtitle: String, api_key: String) -> String {
    let chunk_size = env!("SUBTITLE_LINE_CHUNKS").parse().unwrap_or(200);

    let chunks = split_into_chunks(subtitle.as_str(), chunk_size);

    let mut res_string = String::new();

    for (i, chunk) in chunks.iter().enumerate() {
        LOGGER.info(format!("Start translate chunk {}/{}\n", i + 1, chunks.len()).as_str());
        let response = get_translate(&api_key, chunk, 1);

        if response == "Error" {
            return "Error".to_owned();
        }

        let resp: GeminiResponse = serde_json::from_str(&response).unwrap();
        let lines = extract_prefixed_lines(&resp.candidates[0].content.parts[0].text);

        if i > 0 {
            res_string.push('\n');
        }
        res_string.push_str(lines.as_str());
    }

    res_string
}

fn get_translate(api_key: &str, subtitle: &String, req_num: u8) -> String {
    let prompt = build_translation_prompt(&subtitle);
    match gemini_api(&api_key, &prompt) {
        Ok(res) => return res,
        Err(error) => {
            LOGGER.error(format!("Error: {}\n", error).as_str());
            let retry_err = env!("RETRY_ERROR").parse().unwrap_or(5);
            if req_num <= retry_err {
                LOGGER.warning(&format!(
                    "Trying again to get translation (attempt {}/{})\n",
                    req_num, retry_err
                ));
                return get_translate(api_key, subtitle, req_num + 1);
            } else {
                LOGGER.error("Max retries reached, giving up.\n");
                return "Error".to_string();
            }
        }
    };
}

fn gemini_api(api_key: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let body = GeminiRequestBody {
        contents: vec![Content {
            parts: vec![Part {
                text: prompt.to_owned(),
            }],
        }],
    };

    let client = blocking::Client::new();
    let resp = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()?;

    let status = resp.status();
    let text = resp.text()?;

    if !status.is_success() {
        Err(format!("Request failed ({}): {}", status, text).into())
    } else {
        Ok(text)
    }
}

pub fn build_translation_prompt(input: &str) -> String {
    format!("**System Role:**  
You are a specialized English-to-Persian technical translation engine. Your job is to translate any English text into Persian **line by line**, while preserving specified technical terms and the exact original line structure.

**Instructions:**  
1. **Line Number Prefix:**  
   - If a line begins with a numeric prefix followed by an underscore (e.g. `0_`, `1_`, `2_`, `3_`), keep that prefix unchanged at the start of the output line.  
2. **Technical Elements Preservation:**  
   Identify and leave unchanged any of the following within each line’s content:  
   - **Programming Keywords:** if, else, for, while, return, function, class, ... etc.  
   - **Technology Proper Nouns:** Rust, Python, Linux, Mozilla, ... etc.  
   - **Symbols & Operators:** +, -, *, /, =, ==, {{}}, (), [], ... etc.  
3. **Translation:**  
   - Translate everything else into clear, fluent Persian.  
4. **Reconstruction:**  
   - The output **must** contain exactly the same number of lines, each line matching the input line’s length (excluding prefix) as closely as possible. Do not add, remove, merge, or split lines.  
5. **Plain Text Output:**  
   - Do **not** wrap your response in a code block.  
   - Do **not** include any words, explanations, or messages before, after, or between the translated lines—deliver only the translated lines themselves.

---

**Input Text for Translation:**  
(The lines below are the exact text you must translate according to the above rules. Do not translate anything outside this block.)

```
{}
```
", input)
}

pub fn extract_prefixed_lines(input: &String) -> String {
    let prefix_re = Regex::new(r"^\d+_").unwrap();

    let mut result_lines = Vec::new();
    for line in input.lines() {
        let trimmed = line.trim();
        if prefix_re.is_match(trimmed) {
            result_lines.push(trimmed.to_string());
        }
    }

    result_lines.join("\n")
}

fn split_into_chunks(text: &str, chunk_size: usize) -> Vec<String> {
    text.lines()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.join("\n"))
        .collect()
}

// region GeminiRequestBody
#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct GeminiRequestBody {
    contents: Vec<Content>,
}
// endregion

// region GeminiResponse
#[derive(Deserialize, Debug)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
    pub usageMetadata: UsageMetadata,
    pub modelVersion: String,
}

#[derive(Deserialize, Debug)]
pub struct Candidate {
    pub content: ContentContainer,
    pub finishReason: String,
    pub avgLogprobs: f64,
}

#[derive(Deserialize, Debug)]
pub struct ContentContainer {
    pub parts: Vec<TextPart>,
    pub role: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TextPart {
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct UsageMetadata {
    pub promptTokenCount: usize,
    pub candidatesTokenCount: usize,
    pub totalTokenCount: usize,
    pub promptTokensDetails: Vec<TokenDetail>,
    pub candidatesTokensDetails: Vec<TokenDetail>,
}

#[derive(Deserialize, Debug)]
pub struct TokenDetail {
    pub modality: String,
    pub tokenCount: usize,
}
// endregion
