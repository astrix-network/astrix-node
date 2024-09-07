use crate::result::Result;
use js_sys::BigInt;
use astrix_consensus_core::network::{NetworkType, NetworkTypeT};
use wasm_bindgen::prelude::*;
use workflow_wasm::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "bigint | number | HexString")]
    #[derive(Clone, Debug)]
    pub type ISompiToAstrix;
}

/// Convert a Astrix string to Sompi represented by bigint.
/// This function provides correct precision handling and
/// can be used to parse user input.
/// @category Wallet SDK
#[wasm_bindgen(js_name = "astrixToSompi")]
pub fn astrix_to_sompi(astrix: String) -> Option<BigInt> {
    crate::utils::try_astrix_str_to_sompi(astrix).ok().flatten().map(Into::into)
}

///
/// Convert Sompi to a string representation of the amount in Astrix.
///
/// @category Wallet SDK
///
#[wasm_bindgen(js_name = "sompiToAstrixString")]
pub fn sompi_to_astrix_string(sompi: ISompiToAstrix) -> Result<String> {
    let sompi = sompi.try_as_u64()?;
    Ok(crate::utils::sompi_to_astrix_string(sompi))
}

///
/// Format a Sompi amount to a string representation of the amount in Astrix with a suffix
/// based on the network type (e.g. `AIX` for mainnet, `TAIX` for testnet,
/// `SAIX` for simnet, `DAIX` for devnet).
///
/// @category Wallet SDK
///
#[wasm_bindgen(js_name = "sompiToAstrixStringWithSuffix")]
pub fn sompi_to_astrix_string_with_suffix(sompi: ISompiToAstrix, network: &NetworkTypeT) -> Result<String> {
    let sompi = sompi.try_as_u64()?;
    let network_type = NetworkType::try_from(network)?;
    Ok(crate::utils::sompi_to_astrix_string_with_suffix(sompi, &network_type))
}
