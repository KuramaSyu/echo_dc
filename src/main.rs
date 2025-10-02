use clap::{Parser, Subcommand};
use serenity::builder::ExecuteWebhook;
use serenity::http::Http;
use serenity::model::webhook::Webhook;
mod utils;
use utils::Config;

#[derive(Parser, Debug)]
#[command(name = "echo_dc")]
#[command(about = "Echo - but for Discord instead of the console")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Send a message to a Discord channel
    Send {
        /// The message to send
        message: String,
        /// The channel ID to send the message to
        channel_id: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Send {
            message,
            channel_id,
        } => {}
    }
}

async fn run_send(message: &str, channel_id: &str) {
    // Just a mock async operation
    let config = Config::from_etc().expect("Failed to load config");
    println!("🚀 Sending message to channel {}: {}", channel_id, message);
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("✅ Done.");
}

async fn run_main(config_path: &str) {
    // Just a mock async operation
    println!("🚀 Starting EchoDC with config at: {}", config_path);
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("✅ Done.");
}

async fn send_webhook() {
    // You don't need a token when you are only dealing with webhooks.
    let http = Http::new("");
    let webhook = Webhook::from_url(&http, "https://discord.com/api/webhooks/133742013374206969/hello-there-oPNtRN5UY5DVmBe7m1N0HE-replace-me-Dw9LRkgq3zI7LoW3Rb-k-q")
        .await
        .expect("Replace the webhook with your own");

    let builder = ExecuteWebhook::new()
        .content("hello there")
        .username("Webhook test");
    webhook
        .execute(&http, false, builder)
        .await
        .expect("Could not execute webhook.");
}
