//! Async Rust library for GigaChat REST API based on GigaChatAPI spec.
//!
//! ## Creating client
//!
//! ```
//! use async_gigachat::{client::Client, config::GigaChatConfig};
//!
//! // Loading environment variables from the .env file
//! dotenv::dotenv().ok();
//!
//! // Create a GigaChat client with authorization token from env var GIGACHAT_AUTH_TOKEN with scope env var GIGACHAT_API_SCOPE and default auth url, base url.
//! let client = Client::new();
//!
//! // Above is shortcut for
//! let config = GigaChatConfig::default();
//! let client = Client::with_config(config);
//!
//! // OR use authorization token and scope from different source
//! let auth_token = "YTAxNj...";
//! let config = GigaChatConfig::builder()
//!     .auth_token(auth_token.into())
//!     .scope("GIGACHAT_API_PERS".into())
//!     .build();
//!
//! let client = Client::with_config(config);
//!
//! // Use custom auth url or base url
//! let config = GigaChatConfig::builder()
//!     .auth_url("https://myhost.com/api/v1/oauth".into())
//!     .api_base_url("https://myhost.com/api/v2".into())
//!     .build();
//!
//! let client = Client::with_config(config);
//! ```
//!
//! ## Making requests
//!
//!```
//!# tokio_test::block_on(async {
//!
//! use async_gigachat::{
//!     chat::{Chat, ChatCompletionRequestBuilder, ChatMessage, Role},
//!     client::Client,
//!     config::GigaChatConfig,
//! };
//!
//! // Loading environment variables from the .env file
//! dotenv::dotenv().ok();
//!
//! // Create client
//! let client = Client::new();
//!
//! // Create request using builder pattern
//! let request = ChatCompletionRequestBuilder::default()
//!     .messages(vec![ChatMessage {
//!         role: Role::User,
//!         content: "Hey, how's it going?".into(),
//!     }])
//!     .model("GigaChat:latest".to_owned())
//!     .build()
//!     .unwrap();
//!
//! // Call API
//! let response = Chat::new(client)
//!     .completion(request)
//!     .await
//!     .unwrap();
//!
//! println!("{}", response.choices.get(0).unwrap().message.content);
//! # });
//!```
//!
//! ## Examples
//! For full working examples for all supported features see [examples](https://github.com/xsayler/async-gigachat/tree/main/examples) directory in the repository.
//!
pub mod api;
pub mod chat;
pub mod client;
pub mod config;
pub mod model;
