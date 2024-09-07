use crate::error::Error;
use crate::result::Result;
use astrix_consensus_core::constants::SOMPI_PER_ASTRIX;
use std::fmt::Display;

pub fn try_parse_required_nonzero_astrix_as_sompi_u64<S: ToString + Display>(astrix_amount: Option<S>) -> Result<u64> {
    if let Some(astrix_amount) = astrix_amount {
        let sompi_amount = astrix_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Astrix amount is not valid: '{astrix_amount}'")))?
            * SOMPI_PER_ASTRIX as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Astrix amount is not valid: '{astrix_amount}'"))
        } else {
            let sompi_amount = sompi_amount as u64;
            if sompi_amount == 0 {
                Err(Error::custom("Supplied required astrix amount must not be a zero: '{astrix_amount}'"))
            } else {
                Ok(sompi_amount)
            }
        }
    } else {
        Err(Error::custom("Missing Astrix amount"))
    }
}

pub fn try_parse_required_astrix_as_sompi_u64<S: ToString + Display>(astrix_amount: Option<S>) -> Result<u64> {
    if let Some(astrix_amount) = astrix_amount {
        let sompi_amount = astrix_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Astrix amount is not valid: '{astrix_amount}'")))?
            * SOMPI_PER_ASTRIX as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Astrix amount is not valid: '{astrix_amount}'"))
        } else {
            Ok(sompi_amount as u64)
        }
    } else {
        Err(Error::custom("Missing Astrix amount"))
    }
}

pub fn try_parse_optional_astrix_as_sompi_i64<S: ToString + Display>(astrix_amount: Option<S>) -> Result<Option<i64>> {
    if let Some(astrix_amount) = astrix_amount {
        let sompi_amount = astrix_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_e| Error::custom(format!("Supplied Astrix amount is not valid: '{astrix_amount}'")))?
            * SOMPI_PER_ASTRIX as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Astrix amount is not valid: '{astrix_amount}'"))
        } else {
            Ok(Some(sompi_amount as i64))
        }
    } else {
        Ok(None)
    }
}
