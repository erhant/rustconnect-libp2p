use libp2p_rustconnect::ChatClient;
use std::env;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _ = dotenvy::dotenv(); // load .env file if it exists
    env_logger::builder()
        .filter(None, log::LevelFilter::Off)
        .filter_module("libp2p_rustconnect", log::LevelFilter::Info)
        .filter_module("libp2p", log::LevelFilter::Error)
        .parse_default_env() // reads RUST_LOG variable
        .init();

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
        // if we get this as input, exit gracefully
        const EXIT_MSG: &str = "exit";

        let mut rl = rustyline::DefaultEditor::new().unwrap();
        println!("Type a message and press ENTER to publish it to the network.");
        println!("Type 'exit' to close the client.");
        while !cancellation.is_cancelled() {
            if let Ok(line) = rl.readline("") {
                if line.is_empty() {
                    continue;
                }
                if line.eq(EXIT_MSG) {
                    // this will cancel the client too
                    cancellation.cancel();
                    break;
                }
                if sender.send(line).is_err() {
                    log::error!("Error while sending message to the client");
                    break;
                }
            }
        }
    });

    // run the chat client (blocking here)
    client.run(port).await?;

    // wait for the reader task to finish, since client is finished too at this point
    if let Err(e) = reader_handle.await {
        log::error!("Error while waiting for reader: {}", e);
    }

    Ok(())
}
