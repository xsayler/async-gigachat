use anyhow::Result;
use serde::Deserialize;

use crate::client::Client;

#[derive(Deserialize, Debug)]
pub struct ModelListResponse {
    pub data: Vec<ModelResponse>,
    pub object: String,
}

#[derive(Deserialize, Debug)]
pub struct ModelResponse {
    pub id: String,
    pub object: String,
    pub owned_by: String,
}

pub struct Models {
    client: Client,
}

impl Models {
    pub fn new(client: Client) -> Self {
        Models { client }
    }

    pub async fn list(self) -> Result<ModelListResponse> {
        self.client.get("/models").await
    }
}
