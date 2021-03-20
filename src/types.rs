use std::str::FromStr;

use crate::utils::unix_time;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct VaultPerformance {
    pub gain_last_check: BigDecimal,
    pub gain_past_hour: BigDecimal,
    pub apy_past_hour: BigDecimal,
    pub gain_past_day: BigDecimal,
    pub apy_past_day: BigDecimal,
    pub gain_past_week: BigDecimal,
    pub apy_past_week: BigDecimal,
    pub gain_past_month: BigDecimal,
    pub apy_past_month: BigDecimal,
}

#[derive(Debug, Clone)]
pub struct UserVaultHoldings {
    pub timestamp: u64,
    pub price_per_share: BigDecimal,
    pub cdai: BigDecimal,
    pub cusdc: BigDecimal,
    pub cboth: BigDecimal,
    pub dai: BigDecimal,
    pub usdc: BigDecimal,
    pub both: BigDecimal,
}

#[derive(Debug, Derivative, Serialize, Deserialize, Clone)]
#[derivative(PartialEq, Hash)]
pub struct DbUserVaultHoldings {
    #[derivative(PartialEq = "ignore")]
    pub timestamp: u64,
    pub price_per_share: String,
    pub cdai: String,
    pub cusdc: String,
    pub cboth: String,
    pub dai: String,
    pub usdc: String,
    pub both: String,
}

impl UserVaultHoldings {
    #[allow(dead_code)]
    pub fn zero() -> UserVaultHoldings {
        UserVaultHoldings {
            timestamp: unix_time(),
            price_per_share: BigDecimal::from(0),
            cdai: BigDecimal::from(0),
            cusdc: BigDecimal::from(0),
            cboth: BigDecimal::from(0),
            dai: BigDecimal::from(0),
            usdc: BigDecimal::from(0),
            both: BigDecimal::from(0),
        }
    }
}

impl From<&UserVaultHoldings> for DbUserVaultHoldings {
    fn from(val: &UserVaultHoldings) -> Self {
        DbUserVaultHoldings {
            timestamp: val.timestamp,
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
            timestamp: val.timestamp,
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
