use std::io::{stdin, stdout, Write};

use async_gigachat::{
    chat::{Chat, ChatCompletionRequestBuilder, ChatMessage, ChatMessageBuilder, Role},
    client::Client,
    config::GigaChatConfig,
    result::Result,
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = GigaChatConfig::default();

    let client: Client = Client::with_config(config);

    let mut messages: Vec<ChatMessage> = vec![];

    println!("help: type :q to end the chat\n");

    let mut lock = stdout().lock();

    loop {
        let mut buffer = String::new();

        print!("user: ");
        stdout().flush()?;

        stdin().read_line(&mut buffer)?;

        let buffer = buffer.trim();

        if buffer.eq(":q") {
            break;
        }

        messages.push(
            ChatMessageBuilder::default()
                .role(Role::User)
                .content(buffer)
                .build()?,
        );

        let request = ChatCompletionRequestBuilder::default()
            .messages(messages.clone())
            .model("GigaChat:latest")
            .stream(true)
            .build()?;

        let mut stream = Chat::new(client.clone()).completion_stream(request).await?;

        print!("assistant: ");
        stdout().flush()?;

        let mut message = String::default();

        while let Some(response) = stream.next().await {
            match response {
                Ok(resp) => {
                    for choice in resp.choices.iter() {
                        print!("{}", choice.delta.content);
                        stdout().flush()?;
                        message = format!("{} {}", message, choice.delta.content);
                    }
                }
                Err(e) => {
                    let _ = writeln!(lock, "Error: {:?}", e);
                }
            };
        }

        print!("\n");
        stdout().flush()?;

        messages.push(
            ChatMessageBuilder::default()
                .role(Role::Assistant)
                .content(message.clone())
                .build()?,
        );
    }

    Ok(())
}
