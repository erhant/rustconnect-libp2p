use libp2p_rustconnect::ChatClient;
use std::env;
use tokio_util::sync::CancellationToken;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _ = dotenvy::dotenv(); // load .env file if it exists
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::OFF.into())
                .from_env()?
                .add_directive("libp2p_rustconnect=info".parse()?),
        )
        .try_init();

    // read port from env, defaults to 0
    const DEFAULT_PORT: u16 = 0;
    let port = env::var("PORT")
        .unwrap_or_else(|_| DEFAULT_PORT.to_string())
        .parse::<u16>()
        .unwrap_or(DEFAULT_PORT);

    let cancellation = CancellationToken::new();
    let (mut client, sender) = ChatClient::new(cancellation.clone())?;

    // spawn a task to read line and send messages
    let reader_handle = tokio::spawn(async move {
        let mut rl = rustyline::DefaultEditor::new().expect("could not create rustyline editor");
        println!("Type 'exit' to quit.");
        while !cancellation.is_cancelled() {
            if let Ok(line) = rl.readline("") {
                if line.is_empty() {
                    continue;
                }
                if line.eq("exit") {
                    // this will cancel the client too
                    cancellation.cancel();
                    break;
                }
                if sender.send(line).is_err() {
                    tracing::error!("Error while sending message to the client");
                    break;
                }
            }
        }
    });

    // run the chat client (blocking here)
    client.run(port).await?;

    // wait for the reader task to finish, since client is finished too at this point
    if let Err(e) = reader_handle.await {
        tracing::error!("Error while waiting for reader: {}", e);
    }

    Ok(())
}
