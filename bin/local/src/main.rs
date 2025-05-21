mod writer;
mod cli;

use fusion_streamer::client::FusionClient;
use futures_util::StreamExt;
use eyre::Result;


#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = cli::parse_cli_args();
    
    let endpoint = get_endpoint(args.endpoint)?;
    let auth_key = get_auth_key(args.auth_key)?;
    let network_id = args.network_id;

    let writer = args.out_path
        .map(|out_path| writer::JsonlWriter::new(out_path));

    tracing::info!("Connecting to 1inch Fusion API on network {}", network_id);
    
    let client = FusionClient::new(endpoint, network_id, auth_key);
    let mut stream = client.into_stream().await?;
    
    tracing::info!("Connection established. Listening for events...");
    
    while let Some(msg) = stream.next().await {
        match msg {
            Ok(event) => {
                tracing::info!("Received new Fusion event");
                if let Some(writer) = &writer {
                    writer.write(&event)?;
                } else {
                    println!("{event:#?}");
                }
            }
            Err(e) => {
                tracing::error!("Error: {e}");
            }
        }
    }
    Ok(())
}

fn get_endpoint(cli_endpoint: Option<String>) -> Result<url::Url> {
    match cli_endpoint {
        Some(endpoint) => Ok(endpoint.parse()?),
        None => {
            std::env::var("ONEINCH_ENDPOINT")?
                .parse()
                .map_err(Into::into)
        }
    }
}

fn get_auth_key(cli_auth_key: Option<String>) -> Result<String> {
    match cli_auth_key {
        Some(key) => Ok(key),
        None => {
            let _ = dotenv::dotenv();
            
            let auth_key = std::env::var("ONEINCH_API_TOKEN")
                .map_err(|_| eyre::eyre!("Missing authentication key. Provide via --auth-key or set ONEINCH_API_TOKEN environment variable"))?;
            
            Ok(auth_key)
        }
    }
}