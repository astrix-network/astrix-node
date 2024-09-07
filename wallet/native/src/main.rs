use astrix_cli_lib::{astrix_cli, TerminalOptions};

#[tokio::main]
async fn main() {
    let result = astrix_cli(TerminalOptions::new().with_prompt("$ "), None).await;
    if let Err(err) = result {
        println!("{err}");
    }
}