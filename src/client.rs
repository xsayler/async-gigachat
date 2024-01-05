use std::{
    pin::Pin,
    time::{SystemTime, UNIX_EPOCH},
};

use futures::{stream::StreamExt, Stream};
use log::debug;
use reqwest::Request;
use reqwest_eventsource::{Event, EventSource, RequestBuilderExt};
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

use crate::{
    api::{AccessToken, ErrorResponse},
    config::GigaChatConfig,
    errors::GigaChatError,
    result::Result,
};

#[derive(Clone, Default)]
pub struct Client {
    http_client: reqwest::Client,
    config: GigaChatConfig,
    access_token: Option<AccessToken>,
}

impl Client {
    pub fn new() -> Self {
        Client {
            http_client: reqwest::Client::new(),
            ..Default::default()
        }
    }

    pub fn with_config(config: GigaChatConfig) -> Self {
        Client {
            http_client: reqwest::Client::new(),
            config,
            ..Default::default()
        }
    }

    async fn get_access_token(&mut self) -> Result<AccessToken> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

        if let Some(access_token) = &self.access_token {
            if now < access_token.expires_at as u128 {
                return Ok(access_token.to_owned());
            }
        }

        let new_access_token = self.retrive_access_token().await?;

        self.access_token = Some(new_access_token.clone());

        Ok(new_access_token)
    }

    fn _invalidate_access_token(mut self) -> Result<()> {
        self.access_token = Default::default();

        Ok(())
    }

    async fn retrive_access_token(&mut self) -> Result<AccessToken> {
        let request_id = Uuid::new_v4();

        let response = self
            .http_client
            .post(self.config.auth_url.clone())
            .header("RqUID", request_id.to_string())
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .bearer_auth(self.config.auth_token.clone())
            .body(format!("scope={}", self.config.scope))
            .send()
            .await?;

        match response.error_for_status_ref() {
            Ok(_) => (),
            Err(error) => {
                let error_response: ErrorResponse = response.json().await?;
                log::error!("Error getting access token: {}", error);
                return Err(GigaChatError::HttpError(format!(
                    "Error getting access token: {}",
                    error_response.message
                )));
            }
        };

        let access_token: AccessToken = response.json().await?;

        Ok(access_token)
    }

    pub async fn get<O>(mut self, path: &str) -> Result<O>
    where
        O: DeserializeOwned,
    {
        let request = self
            .http_client
            .get(format!("{}{}", self.config.api_base_url, path))
            .bearer_auth(self.get_access_token().await?.access_token)
            .build()?;

        self.execute(request).await
    }

    pub async fn post<I, O>(mut self, path: &str, body: I) -> Result<O>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let request = self
            .http_client
            .post(format!("{}{}", self.config.api_base_url, path))
            .bearer_auth(self.get_access_token().await?.access_token)
            .json(&body)
            .build()?;

        self.execute(request).await
    }

    pub async fn post_stream<I, O>(
        mut self,
        path: &str,
        body: I,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<O>>>>>
    where
        I: Serialize,
        O: DeserializeOwned + Send + 'static,
    {
        let request = self
            .http_client
            .post(format!("{}{}", self.config.api_base_url, path))
            .bearer_auth(self.get_access_token().await?.access_token)
            .json(&body)
            .eventsource()
            .unwrap();

        Ok(self.stream(request).await)
    }

    pub async fn execute<R>(self, request: Request) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let response = self.http_client.execute(request).await?;

        match response.error_for_status_ref() {
            Ok(_) => (),
            Err(error) => {
                // let error_response: ErrorResponse = response.json().await?;
                log::error!("Error execute request: {}", error);
                return Err(GigaChatError::HttpError(format!(
                    "Error execute request: {}",
                    error
                )));
            }
        };

        let response_text = response.text().await?;

        debug!("response:\n{}", response_text);

        let result: R = serde_json::from_str(&response_text)?;

        Ok(result)
    }

    async fn stream<O>(
        self,
        mut event_source: EventSource,
    ) -> Pin<Box<dyn Stream<Item = Result<O>>>>
    where
        O: DeserializeOwned + Send + 'static,
    {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(event) = event_source.next().await {
                match event {
                    Ok(event) => match event {
                        Event::Open => continue,
                        Event::Message(message) => {
                            let data = message.data;

                            if data == "[DONE]" {
                                break;
                            }

                            let result: Result<O> = serde_json::from_str(&data)
                                .map_err(|error| GigaChatError::StreamError(error.to_string()));

                            if let Err(error) = tx.send(result) {
                                log::error!("Error sending event: {}", error);
                                break;
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("Error getting event: {}", error);
                        tx.send(Err(GigaChatError::StreamError(error.to_string())))
                            .unwrap();
                        break;
                    }
                }
            }

            event_source.close();
        });

        Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(rx))
    }
}
