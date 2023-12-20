use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::stream::StreamExt; // For StreamExt which provides 'next' method
use std::process::Command;
use url::Url;

#[tokio::main]
async fn main() {
    let server_url = Url::parse("ws://127.0.0.1:8080/ws/").expect("Failed to parse WebSocket URL");

    let (ws_stream, _) = connect_async(server_url)
        .await
        .expect("Failed to connect to WebSocket");

    println!("WebSocket client connected");

    let (_, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) if text == "trigger_script" => {
                run_powershell_script();
            }
            _ => (),
        }
    }
}

fn run_powershell_script() {
    let output = Command::new("powershell")
        .arg("./scripts/script1.ps1")  // Ensure this path is correct
        .output()
        .expect("Failed to execute script");

    println!("Script output: {:?}", output);
}
