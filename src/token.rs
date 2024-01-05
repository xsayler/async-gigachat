use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::errors::GigaChatError;
use crate::result::Result;

#[derive(Debug, Serialize, Builder)]
#[builder(setter(into, strip_option))]
pub struct TokenCountRequest {
    pub model: String,
    pub input: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenCountResponse {
    pub object: String,
    pub tokens: usize,
    pub characters: usize,
}

impl From<TokenCountRequestBuilderError> for GigaChatError {
    fn from(error: TokenCountRequestBuilderError) -> Self {
        GigaChatError::SystemError(error.to_string())
    }
}

pub struct Tokens {
    client: Client,
}

impl Tokens {
    pub fn new(client: Client) -> Self {
        Tokens { client }
    }

    pub async fn get_token_count(
        self,
        request: TokenCountRequest,
    ) -> Result<Vec<TokenCountResponse>> {
        let response = self.client.post("/tokens/count", request).await?;

        Ok(response)
    }
}
