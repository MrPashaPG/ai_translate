#![allow(non_snake_case)]
#![allow(dead_code)]

use regex::Regex;
use reqwest::blocking;
use serde::{Deserialize, Serialize};

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
    pub role: Option<String>, // sometimes present
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

///////////////////////////////////////////////////

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct RequestBody {
    contents: Vec<Content>,
}

pub fn translate_subtitle(subtitle: String, api_key: String) -> String {
    let prompt = build_translation_prompt(&subtitle);
    let response = match gemini_api(&api_key, &prompt) {
        Ok(res) => res,
        Err(error) => {
            println!("Error: {}", error);
            return "Error".to_string();
        }
    };

    let resp: GeminiResponse = serde_json::from_str(&response).unwrap();

    extract_prefixed_lines(&resp.candidates[0].content.parts[0].text)
}

fn gemini_api(api_key: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let body = RequestBody {
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
    let prompt_template = r#"You are a specialized English-to-Persian technical translation engine. Your job is to translate any English text into Persian **line by line**, while preserving specified technical terms and the original line structure exactly.

**Instructions:**  
1. **Line Number Prefix:**  
   - If a line begins with a numeric prefix followed by an underscore (e.g. `0_`, `12_`), keep that prefix unchanged at the start of the output line.  
2. **Technical Elements Preservation:**  
   Identify and leave unchanged any of the following within each line’s content:  
   - **Programming Keywords:** `if`, `else`, `for`, `while`, `return`, `function`, `class`, etc.  
   - **Technology Proper Nouns:** `Rust`, `Python`, `Linux`, `Mozilla`, etc.  
   - **Symbols & Operators:** `+`, `-`, `*`, `/`, `=`, `==`, `{}`, `()`, `[]`, etc.  
3. **Translation:**  
   - Translate all other words and phrases into clear, fluent Persian.  
4. **Reconstruction:**  
   - Output exactly the same number of lines as the input, each with its prefix (if any) followed by the mixed Persian/English content.  
5. **Code Block:**  
   - Wrap the entire translated output in a single Markdown code block (```…```).  
6. **No Extras:**  
   - Do not add, remove, merge, or split lines. Do not include any text outside the code block.

---

**Input Text for Translation:**  
(The lines below are the exact text you must translate according to the above rules. Do not translate anything outside this block.)

```
{}
```
"#;

    prompt_template.replace("{}", input)
}

pub fn extract_prefixed_lines(input: &String) -> String {
    let code_block_re = Regex::new(r"(?s)```(.*?)```").unwrap();
    let prefix_re = Regex::new(r"^\d+_").unwrap();

    let mut result_lines = Vec::new();
    for cap in code_block_re.captures_iter(input) {
        let block = &cap[1];
        for line in block.lines() {
            let trimmed = line.trim();
            if prefix_re.is_match(trimmed) {
                result_lines.push(trimmed.to_string());
            }
        }
    }

    result_lines.join("\n")
}
