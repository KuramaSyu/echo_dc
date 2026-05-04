use clap::{Parser, Subcommand};
use serenity::all::{CreateEmbed, Embed, Message};
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

    /// The content of the message to send
    #[arg(global = true, help = "Content of the message to send")]
    content: Option<Vec<String>>,

    /// The title of the message to send
    #[arg(global = true, long, short='t', help = "Title of the message to send")]
    title: Option<String>,

    /// The color of the embed in hex (with #)
    #[arg(global = true, long, short='C', help = "Color of the embed in hex (without the #)", default_value = "0x6A5ACD")]
    color: Option<String>,
    

    #[command(subcommand)]
    info: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Print information about the config file and its webhooks
    Info,
}

/// Represents the available commandline flags
struct MessageContent {
    title: Option<String>,
    content: Option<String>,
    color: u32, // hex for slateblue: 0x6A5ACD,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // checks and if needed creates config
    Config::ensure_file_exists(&Config::default_path());
    let config = match Config::from_etc() {
        Ok(c) => c,
        Err(e) => {
            let default_config = Config::template();
            default_config.to_etc().unwrap();
            default_config
        }
    };

    if let Some(Commands::Info) = cli.info {
        println!("Config file location: {}", Config::default_path());
        println!("Available webhooks: {:?}", config.webhooks.keys().collect::<Vec<&String>>());
        return;
    }
    let webhook = match config.get_webhook(&cli.webhook_name.clone().unwrap()) {
        Some(v) => v,
        None => {
            eprintln!(
                "Webhook '{}' not found in {}. Available webhooks: {:?}",
                cli.webhook_name.unwrap(),
                Config::default_path(),
                config.webhooks.keys().collect::<Vec<&String>>()
            );
            std::process::exit(1);
        }
    };
    let message = cli.content.unwrap_or_default().join(" ");
    if message.is_empty() {
        eprintln!("No message provided to send.");
        std::process::exit(1);
    }

    let mut message_content = MessageContent {
        title: cli.title,
        content: Some(message),
        color: u32::from_str_radix(
            cli.color
                .unwrap_or_else(|| "0xFFFFFF".to_string())
                .trim_start_matches("0x")
                .trim_start_matches('#'),
            16
        ).unwrap(),
    };
      

    // if title is > 256 chars, add it to content
    if let Some(title) = &message_content.title {
        if title.len() > 256 {
            message_content.content = Some(format!("**{}**\n\n{}", title, message_content.content.unwrap_or_default()));
            message_content.title = None;
        }
    }

    // add 0x prefix to color if not present
    send_webhook(&message_content, &webhook).await;
}

async fn send_webhook(message: &MessageContent, webhook_url: &str) {
    // You don't need a token when you are only dealing with webhooks.
    let http = Http::new("");
    let webhook = Webhook::from_url(&http, webhook_url)
        .await
        .expect("Replace the webhook with your own");

    let builder = ExecuteWebhook::new()
        .username("Vuekos Echo")
        .embed(
            CreateEmbed::new()
            .title(message.title.as_deref().unwrap_or("Echo - Message Sent"))
            .description(message.content.as_deref().unwrap_or(""))
            .color(message.color)
        );
    webhook
        .execute(&http, false, builder)
        .await
        .expect("Could not execute webhook.");
}
