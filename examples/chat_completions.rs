use anyhow::Ok;
use async_gigachat::{
    chat::{Chat, ChatCompletionRequestBuilder, ChatMessageBuilder, Role},
    client::Client,
    config::GigaChatConfig,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = GigaChatConfig::default();

    let client: Client = Client::with_config(config);

    let question = ChatMessageBuilder::default()
        .role(Role::User)
        .content("Hey, how's it going?".into())
        .build()?;

    let request = ChatCompletionRequestBuilder::default()
        .messages(vec![question.clone()])
        .model("GigaChat:latest".to_owned())
        .build()?;

    let response = Chat::new(client).completion(request).await?;

    println!("{}: {}", question.role, question.content);
    println!(
        "{}: {}",
        response.choices.get(0).unwrap().message.role,
        response.choices.get(0).unwrap().message.content
    );

    Ok(())
}
