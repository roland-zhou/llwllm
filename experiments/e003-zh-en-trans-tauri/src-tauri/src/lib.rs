// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use async_openai::config::OpenAIConfig;
use async_openai::types::CreateChatCompletionRequest;
use async_openai::types::ChatCompletionRequestSystemMessage;
use async_openai::types::ChatCompletionRequestUserMessage;
use async_openai::types::ChatCompletionRequestUserMessageContent;
use async_openai::types::ChatCompletionRequestMessage;
use async_openai::types::Role;
use async_openai::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationError {
    message: String,
}

impl std::fmt::Display for TranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[tauri::command]
async fn translate(text: &str) -> Result<String, TranslationError> {
    if text.trim().is_empty() {
        return Err(TranslationError {
            message: "Please enter some text to translate".to_string(),
        });
    }

    let api_key = env::var("OPENAI_API_KEY").map_err(|_| TranslationError {
        message: "OpenAI API key not found. Please set OPENAI_API_KEY environment variable".to_string(),
    })?;

    let config = OpenAIConfig::new().with_api_key(api_key);
    let client = Client::with_config(config);

    let system_message = ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
        role: Role::System,
        content: "You are a professional translator. Translate between Chinese and English. \
            Provide only the translation without any explanations or notes."
        .to_string(),
        name: None,
    });

    let user_message = ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
        role: Role::User,
        content: ChatCompletionRequestUserMessageContent::Text(text.to_string()),
        name: None,
    });

    let request = CreateChatCompletionRequest {
        model: "gpt-4".to_string(),
        messages: vec![system_message, user_message],
        ..Default::default()
    };

    let response = client
        .chat()
        .create(request)
        .await
        .map_err(|e| TranslationError {
            message: format!("Translation failed: {}", e),
        })?;

    let translation = response
        .choices
        .first()
        .ok_or_else(|| TranslationError {
            message: "No translation received".to_string(),
        })?
        .message
        .content
        .clone()
        .unwrap_or_default();

    Ok(translation)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![translate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
