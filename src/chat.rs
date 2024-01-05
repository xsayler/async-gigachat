use std::{fmt, pin::Pin};

use crate::{errors::GigaChatError, result::Result};
use derive_builder::Builder;
use futures::Stream;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::client::Client;

#[derive(Clone, Serialize, Default, Debug, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_interval: Option<f32>,
}

impl From<ChatCompletionRequestBuilderError> for GigaChatError {
    fn from(error: ChatCompletionRequestBuilderError) -> Self {
        GigaChatError::SystemError(error.to_string())
    }
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize)]
#[builder(setter(into, strip_option))]
pub struct ChatMessage {
    pub role: Option<Role>,
    pub content: String,
}

impl From<ChatMessageBuilderError> for GigaChatError {
    fn from(error: ChatMessageBuilderError) -> Self {
        GigaChatError::SystemError(error.to_string())
    }
}

#[derive(Clone, Serialize, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    Assistant,
    User,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<ChatChoice>,
    pub created: i64,
    pub model: String,
    pub usage: Usage,
    pub object: String,
}

#[derive(Clone, Deserialize)]
pub struct ChatCompletionStreamResponse {
    pub choices: Vec<ChatStreamChoice>,
    pub created: i64,
    pub model: String,
    pub object: String,
}

#[derive(Clone, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
    pub index: u32,
    pub finish_reason: String,
}

#[derive(Clone, Deserialize)]
pub struct ChatStreamChoice {
    pub delta: ChatMessage,
    pub index: u32,
    pub finish_reason: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

pub struct Chat {
    client: Client,
}

impl Chat {
    pub fn new(client: Client) -> Self {
        Chat { client }
    }

    pub async fn completion(
        self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        debug!("request:\n{}", serde_json::to_string_pretty(&request)?);

        let response = self.client.post("/chat/completions", request).await?;

        Ok(response)
    }

    pub async fn completion_stream(
        self,
        request: ChatCompletionRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatCompletionStreamResponse>>>>> {
        debug!("request:\n{}", serde_json::to_string_pretty(&request)?);

        match request.stream {
            Some(true) => (),
            _ => {
                return Err(GigaChatError::InvalidArgument(
                    "When stream is false, use Chat::completion".to_owned(),
                ))
            }
        }

        let response = self
            .client
            .post_stream("/chat/completions", request)
            .await?;

        Ok(response)
    }
}
