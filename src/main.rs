use clap::{Parser, Subcommand};
use serenity::builder::ExecuteWebhook;
use serenity::http::Http;
use serenity::model::webhook::{self, Webhook};
mod utils;
use utils::Config;

#[derive(Parser, Debug)]
#[command(name = "echo_dc")]
#[command(about = "Echo - but for Discord instead of the console")]
struct Cli {
    /// The name of the webhook to use
    #[arg(global = true, help = "Name of the webhook to send to")]
    webhook_name: Option<String>,

    /// The message to send (consumes everything after)
    #[arg(global = true, help = "Message to send", trailing_var_arg = true)]
    message: Option<Vec<String>>,
    // #[command(subcommand)]
    // command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config = Config::from_etc().unwrap_or(Config::template());
    let webhook = match config.get_webhook(&cli.webhook_name.clone().unwrap()) {
        Some(v) => v,
        None => {
            eprintln!(
                "Webhook '{}' not found in config. Available webhooks: {:?}",
                cli.webhook_name.unwrap(),
                config.webhooks.keys().collect::<Vec<&String>>()
            );
            std::process::exit(1);
        }
    };
    let message = cli.message.unwrap_or_default().join(" ");
    if message.is_empty() {
        eprintln!("No message provided to send.");
        std::process::exit(1);
    }
    send_webhook(message, &webhook).await;
}

async fn send_webhook(message: String, webhook_url: &str) {
    // You don't need a token when you are only dealing with webhooks.
    let http = Http::new("");
    let webhook = Webhook::from_url(&http, webhook_url)
        .await
        .expect("Replace the webhook with your own");

    let builder = ExecuteWebhook::new()
        .content(message)
        .username("Vuekos Echo");
    webhook
        .execute(&http, false, builder)
        .await
        .expect("Could not execute webhook.");
}
