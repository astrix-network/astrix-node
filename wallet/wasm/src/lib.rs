use astrix_cli_lib::astrix_cli;
use wasm_bindgen::prelude::*;
use workflow_terminal::Options;
use workflow_terminal::Result;

#[wasm_bindgen]
pub async fn load_astrix_wallet_cli() -> Result<()> {
    let options = Options { ..Options::default() };
    astrix_cli(options, None).await?;
    Ok(())
}
