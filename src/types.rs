use std::str::FromStr;

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct UserVaultHoldings {
    pub price_per_share: BigDecimal,
    pub cdai: BigDecimal,
    pub cusdc: BigDecimal,
    pub cboth: BigDecimal,
    pub dai: BigDecimal,
    pub usdc: BigDecimal,
    pub both: BigDecimal,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbUserVaultHoldings {
    pub price_per_share: String,
    pub cdai: String,
    pub cusdc: String,
    pub cboth: String,
    pub dai: String,
    pub usdc: String,
    pub both: String,
}

impl From<&UserVaultHoldings> for DbUserVaultHoldings {
    fn from(val: &UserVaultHoldings) -> Self {
        DbUserVaultHoldings {
            price_per_share: val.price_per_share.to_string(),
            cdai: val.cdai.to_string(),
            cusdc: val.cusdc.to_string(),
            cboth: val.cboth.to_string(),
            dai: val.dai.to_string(),
            usdc: val.usdc.to_string(),
            both: val.both.to_string(),
        }
    }
}

impl From<&DbUserVaultHoldings> for UserVaultHoldings {
    fn from(val: &DbUserVaultHoldings) -> Self {
        UserVaultHoldings {
            price_per_share: BigDecimal::from_str(&val.price_per_share).unwrap(),
            cdai: BigDecimal::from_str(&val.cdai).unwrap(),
            cusdc: BigDecimal::from_str(&val.cusdc).unwrap(),
            cboth: BigDecimal::from_str(&val.cboth).unwrap(),
            dai: BigDecimal::from_str(&val.dai).unwrap(),
            usdc: BigDecimal::from_str(&val.usdc).unwrap(),
            both: BigDecimal::from_str(&val.both).unwrap(),
        }
    }
}
