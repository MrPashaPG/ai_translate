#![allow(non_snake_case)]
#![allow(dead_code)]

use regex::Regex;
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use std::{env, sync::LazyLock, thread, time::Duration};

use crate::logger::Logger;

static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new("Translator"));

pub fn translate_subtitle(subtitle_text: String, api_key: String) -> Result<String, String> {
    let chunk_size: usize = env!("SUBTITLE_LINE_CHUNKS").parse().unwrap_or(150);
    let max_retries: u8 = env!("MAX_RETRY_ERROR").parse().unwrap_or(3);
    let retry_delay_ms: u64 = env!("RETRY_DELAY_MS").parse().unwrap_or(1000);

    if subtitle_text.trim().is_empty() {
        LOGGER.warning("Input subtitle text is empty. Nothing to translate.");
        return Err("".to_string());
    }

    let chunks = split_into_chunks(&subtitle_text, chunk_size);
    if chunks.is_empty() || chunks.iter().all(|c| c.trim().is_empty()) {
        LOGGER.warning("Subtitle text split into empty chunks. Nothing to translate.");
        return Err("".to_string());
    }

    let mut translated_chunks = Vec::new();
    let total_chunks = chunks.len();

    LOGGER.info(
        format!(
            "Input text split into {} chunks for translation.",
            total_chunks
        )
        .as_str(),
    );

    for (i, chunk) in chunks.iter().enumerate() {
        if chunk.trim().is_empty() {
            LOGGER.info(format!("Chunk {}/{} is empty, skipping.", i + 1, total_chunks).as_str());
            translated_chunks.push("".to_string()); // Add empty string to maintain structure if needed
            continue;
        }
        LOGGER.process(format!("⏳ Translating chunk {} of {}...", i + 1, total_chunks).as_str());

        match attempt_translation_with_retries(
            api_key.as_str(),
            chunk,
            i + 1,
            total_chunks,
            max_retries,
            retry_delay_ms,
        ) {
            Ok(translated_chunk_text) => {
                translated_chunks.push(translated_chunk_text);
                LOGGER.success(
                    format!(
                        "Chunk {} of {} translated successfully.",
                        i + 1,
                        total_chunks
                    )
                    .as_str(),
                );
            }
            Err(e) => {
                LOGGER.error(
                    format!(
                        "❌ Translation of chunk {} of {} failed after {} retries: {}",
                        i + 1,
                        total_chunks,
                        max_retries, // This should be attempt count from the error if available, or max_retries
                        e
                    )
                    .as_str(),
                );
                return Err(format!("Error: {}", e).to_string()); // Propagate a generic error string
            }
        }
    }
    Ok(translated_chunks.join("\n"))
}

fn attempt_translation_with_retries(
    api_key: &str,
    chunk_text: &String,
    chunk_index: usize,
    total_chunks: usize,
    max_retries: u8,
    retry_delay_ms: u64,
) -> Result<String, String> {
    let prompt = build_translation_prompt(chunk_text);
    let mut last_error: String = "Unknown error".to_string();

    for attempt in 1..=max_retries {
        match gemini_api(api_key, &prompt) {
            Ok(response_text) => {
                // Check if response is a GeminiErrorResponse first
                if let Ok(gemini_error) =
                    serde_json::from_str::<GeminiErrorResponse>(&response_text)
                {
                    last_error = format!(
                        "Gemini API Error: (Code: {}) {} - Status: {}",
                        gemini_error.error.code,
                        gemini_error.error.message,
                        gemini_error.error.status
                    );
                    LOGGER.error(last_error.as_str());
                    // Fall through to retry logic if not max_retries
                } else {
                    // Attempt to parse as successful response if not an error object
                    match serde_json::from_str::<GeminiResponse>(&response_text) {
                        Ok(resp) => {
                            if resp.candidates.is_empty()
                                || resp.candidates[0].content.parts.is_empty()
                            {
                                last_error = "API response was successful but did not contain expected content (candidates/parts).".to_string();
                                LOGGER.warning(&last_error);
                                // Fall through to retry
                            } else {
                                // Successfully got translatable text
                                let res_translated =
                                    resp.candidates[0].content.parts[0].text.clone();
                                match check_translated_and_orginal_lines(
                                    &res_translated,
                                    &chunk_text,
                                    chunk_index,
                                    total_chunks,
                                ) {
                                    Ok(res) => return Ok(res),
                                    Err(error) => {
                                        if error == "NotEqual".to_owned() {
                                            last_error = "The count of translated lines did not match the number of lines submitted for translation.".to_string();
                                            LOGGER.warning(&last_error);
                                        } else {
                                            last_error = error;
                                        }
                                    }
                                };
                            }
                        }
                        Err(e) => {
                            // This means the response was not a known GeminiErrorResponse nor a valid GeminiResponse
                            last_error = format!(
                                "Error parsing API response: {}. Raw response (partial): '{}'",
                                e,
                                response_text.chars().take(200).collect::<String>()
                            );
                            LOGGER.error(&last_error);
                            // Fall through to retry
                        }
                    }
                }
            }
            Err(e) => {
                // Network or other pre-API call errors
                last_error = format!("Error communicating with API: {}", e);
                LOGGER.error(&last_error);
                // Fall through to retry
            }
        }

        if attempt < max_retries {
            LOGGER.warning(
                format!(
                    "Retrying translation (attempt {}/{}) after {}ms...",
                    attempt + 1, // Next attempt number
                    max_retries,
                    retry_delay_ms
                )
                .as_str(),
            );
            thread::sleep(Duration::from_millis(retry_delay_ms * attempt as u64));
            // Simple increasing backoff
        }
    }
    Err(format!(
        "Translation failed after {} attempts. Last error: {}",
        max_retries, last_error
    ))
}

fn gemini_api(api_key: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let model_name = "gemini-2.0-flash"; // or "gemini-1.0-pro" or "gemini-1.5-flash-latest" etc.
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model_name, api_key
    );

    let body = GeminiRequestBody {
        contents: vec![Content {
            parts: vec![Part {
                text: prompt.to_owned(),
            }],
        }],
        generation_config: Some(GenerationConfig {
            temperature: Some(2.0), // Adjusted for more deterministic technical translation
            max_output_tokens: Some(8192), // Maximize output tokens
                                    // candidate_count: Some(1), // Default is 1, explicit for clarity
                                    // stop_sequences: None, // No specific stop sequences
                                    // top_p: Some(0.95), // Adjust if needed
                                    // top_k: Some(40),   // Adjust if needed
        }),
    };

    let client = blocking::Client::builder()
        .timeout(Duration::from_secs(360)) // Generous timeout for API
        .build()?;

    LOGGER.debug(
        format!(
            "Sending request to Gemini API. URL: {}, Model: {}",
            url, model_name
        )
        .as_str(),
    );
    // LOGGER.debug(format!("Request body: {}", serde_json::to_string_pretty(&body)?).as_str());

    let resp = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()?;

    let status = resp.status();
    let text = resp.text()?;
    LOGGER.debug(
        format!(
            "API Response Status: {}. Response (partial): {}",
            status,
            text.chars().take(500).collect::<String>()
        )
        .as_str(),
    );

    if !status.is_success() {
        if let Ok(gemini_error) = serde_json::from_str::<GeminiErrorResponse>(&text) {
            Err(format!(
                "Gemini API Error: (Code: {}) {} - Status: {}",
                gemini_error.error.code, gemini_error.error.message, gemini_error.error.status
            )
            .into())
        } else {
            Err(format!("Request failed (Status {}): {}", status, text).into())
        }
    } else {
        Ok(text)
    }
}

pub fn build_translation_prompt(input: &str) -> String {
    format!("You are a translation assistant. When given a text enclosed in triple backticks:
For each line, output exactly one translated line, in the same order and with the same floating-point line number prefix.
Produce a fluent and technically accurate Persian translation. Prioritize translating technical software/programming terms into their common Persian equivalents. Retain in English only essential elements like specific code identifiers (e.g., user_id, calculateTotal, method, string, syntax, ...), programming language names (e.g., Rust, Kotlin, Python, ...), operators, or globally recognized acronyms (e.g., \"HTML\", ...) when their English form is standard in Persian and aids clarity. Text within punctuation should also be translated unless it's one of these essential English elements.
Translate all other words into fluent Persian.
Do not merge, split, add, or remove any lines or line numbers; even if a line contains only one word or is empty, you must reproduce its line number and provide its translation or an empty line as appropriate.
Ensure that no line is left completely untranslated—every line must include at least one translated word where applicable (excluding purely technical identifiers).
Do not output anything before or after the translated lines, and do not wrap the translations in a code block—only the translated lines themselves.

Example input (for illustration only, do not translate this example):
```0.0_Hello World  
0.1_Variable name: x  
1.0_This is a really cool feature, ain't it?
1.1_The system uses a \"FIFO\" queue.
2.0_My 'main_loop' function is bugging out.
2.1_Honestly, that \"whatchamacallit\" component is giving me a headache.
3.0_We need to refactor the legacy code; it's a bit of a spaghetti monster right now, and the 'user_id' field requires \"validation\".
4.0_Consider the [Array.prototype.map()] method.
5.0_This \"quick fix\" is, like, totally not sustainable.
5.1_The variable `count` should be an `i32`.
5.2_Alright, let's get this show on the road!```

Expected output format (illustration only):
0.0_سلام دنیا
0.1_نام variable: x
1.0_این یک ویژگی واقعا باحال است، مگه نه؟
1.1_سیستم از یک صف \"FIFO\" استفاده می‌کند.
2.0_تابع 'main_loop' من ایراد پیدا کرده است.
2.1_راستش رو بخوای، اون قطعه «فلان چیز» داره کلافه‌ام می‌کنه.
3.0_ما باید کد قدیمی را بازآرایی کنیم؛ الان یک کم مثل هیولای اسپاگتی شده، و فیلد 'user_id' نیاز به «اعتبارسنجی» دارد.
4.0_متد [Array.prototype.map()] را در نظر بگیرید.
5.0_این «راه‌حل سریع»، انگار، کاملاً پایدار نیست.
5.1_متغیر `count` باید یک `i32` باشد.
5.2_خیلی خب، بزن بریم!

Now translate the text provided.

```{}```", input)
}

pub fn extract_prefixed_lines(input: &str) -> String {
    // Regex to find lines that START with one or more digits followed by an underscore.
    let prefix_re = Regex::new(r"^\d+.\d+_").unwrap();
    let mut result_lines = Vec::new();

    for line in input.lines() {
        let trimmed_line = line.trim_end(); // Trim only trailing whitespace to preserve leading spaces if any after prefix.
        if prefix_re.is_match(trimmed_line) {
            result_lines.push(trimmed_line.to_string());
        } else if !trimmed_line.is_empty() {
            // This logs lines from the AI response that do NOT conform to the expected "number_" prefix.
            // It could be parts of an error message from the AI, or malformed output.
            LOGGER.debug(
                format!(
                    "Non-prefixed or malformed line from AI (will be ignored): '{}'",
                    trimmed_line
                )
                .as_str(),
            );
        }
    }
    result_lines.join("\n")
}

fn split_into_chunks(text: &str, chunk_size: usize) -> Vec<String> {
    if chunk_size == 0 {
        LOGGER.warning("Chunk size is 0, returning text as a single chunk.");
        return vec![text.to_string()];
    }
    text.lines()
        .map(|s| s.trim_end()) // Trim trailing whitespace from lines before chunking
        .filter(|s| !s.is_empty()) // Remove empty lines before chunking
        .collect::<Vec<&str>>()
        .chunks(chunk_size)
        .map(|chunk_lines| chunk_lines.join("\n"))
        .collect()
}

fn check_translated_and_orginal_lines(
    ai_response: &String,
    orginal_chunk: &String,
    chunk_index: usize,
    total_chunks: usize,
) -> Result<String, String> {
    let extracted_lines = extract_prefixed_lines(&ai_response);
    if extracted_lines.is_empty() && !ai_response.trim().is_empty() {
        LOGGER.warning(format!("Chunk {}/{} translated, but no lines with numeric prefix were extracted. API response (partial): '{}'", chunk_index, total_chunks, ai_response.chars().take(100).collect::<String>()).as_str());
        // Depending on strictness, this could be an error.
        // For now, we are strict and expect prefixed lines.
        let err_msg = format!(
            "Error: Chunk {}/{}: No prefixed lines in translation from API.",
            chunk_index, total_chunks
        );
        LOGGER.error(err_msg.as_str());
        return Err(err_msg);
    } else if extracted_lines.is_empty() && ai_response.trim().is_empty() {
        LOGGER.warning(
            format!(
                "Chunk {}/{} translated, but API response was empty.",
                chunk_index, total_chunks
            )
            .as_str(),
        );
        // This is likely an issue with the API or prompt for this chunk.
        let err_msg = format!(
            "Error: Chunk {}/{}: Empty response from API.",
            chunk_index, total_chunks
        );
        LOGGER.error(err_msg.as_str());
        return Err(err_msg);
    }

    let translated_lines = count_non_empty_lines(&extracted_lines);
    let orginal_lines = count_non_empty_lines(&orginal_chunk);

    if translated_lines == orginal_lines {
        Ok(extracted_lines)
    } else {
        Err("NotEqual".to_owned())
    }
}

pub fn count_non_empty_lines(text: &String) -> usize {
    text.lines().filter(|line| !line.trim().is_empty()).count()
}

// region Gemini API Structs (ensure these match the latest API spec if issues arise)

#[derive(Serialize, Debug)]
struct Part {
    text: String,
}

#[derive(Serialize, Debug)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct GenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<i32>,
    // candidate_count, top_p, top_k can be added here if needed
    // #[serde(skip_serializing_if = "Option::is_none")]
    // candidate_count: Option<i32>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // stop_sequences: Option<Vec<String>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // top_p: Option<f32>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // top_k: Option<i32>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GeminiRequestBody {
    contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GenerationConfig>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeminiResponse {
    #[serde(default = "Vec::new")] // Handle cases where candidates might be missing
    pub candidates: Vec<Candidate>,
    #[serde(default)]
    pub usage_metadata: Option<UsageMetadata>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: ContentContainer,
    #[serde(default)]
    pub finish_reason: Option<String>,
    // Add other fields like 'safetyRatings', 'citationMetadata' if needed and available
}

#[derive(Deserialize, Debug)]
pub struct ContentContainer {
    #[serde(default = "Vec::new")] // Handle cases where parts might be missing
    pub parts: Vec<TextPart>,
    pub role: Option<String>, // Role of the content producer (e.g., "model")
}

#[derive(Deserialize, Debug, Default)] // Default for TextPart if parts array is empty
pub struct TextPart {
    #[serde(default)] // Handle cases where text might be missing in a part
    pub text: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    #[serde(default)]
    pub prompt_token_count: usize,
    #[serde(default)]
    pub candidates_token_count: usize,
    #[serde(default)]
    pub total_token_count: usize,
}

// Structures for handling Gemini API errors specifically
#[derive(Deserialize, Debug)]
struct GeminiErrorResponse {
    error: GeminiErrorDetail,
}

#[derive(Deserialize, Debug)]
struct GeminiErrorDetail {
    code: i32,
    message: String,
    status: String,
    // details: Option<Vec<serde_json::Value>>, // For more detailed error info if provided by API
}
// endregion
