use std::str::FromStr;

use crate::utils::unix_time;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

pub struct VaultIdentifier {
    pub address: String,
    pub vault_name: String,
}

impl VaultIdentifier {
    pub fn new(address: &str, vault_name: &str) -> VaultIdentifier {
        VaultIdentifier {
            address: address.to_owned(),
            vault_name: vault_name.to_owned(),
        }
    }

    pub fn id(&self) -> String {
        format!("{}-{}", self.address, self.vault_name)
    }
}

#[derive(Debug, Clone)]
pub struct DbInfo {
    pub oldest_timestamp: u64,
    pub newest_timestamp: u64,
    pub entry_count: usize,
}

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
    pub usd_1: BigDecimal,
    pub usd_2: BigDecimal,
    pub usd_all: BigDecimal,
}

#[derive(Debug, Derivative, Serialize, Deserialize, Clone)]
#[derivative(PartialEq, Hash)]
pub struct DbUserVaultHoldings {
    #[derivative(PartialEq = "ignore")]
    pub timestamp: u64,
    pub price_per_share: String,
    pub usd_1: String,
    pub usd_2: String,
    pub usd_all: String,
}

impl UserVaultHoldings {
    #[allow(dead_code)]
    pub fn zero() -> UserVaultHoldings {
        UserVaultHoldings {
            timestamp: unix_time(),
            price_per_share: BigDecimal::from(0),
            usd_1: BigDecimal::from(0),
            usd_2: BigDecimal::from(0),
            usd_all: BigDecimal::from(0),
        }
    }
}

impl From<&UserVaultHoldings> for DbUserVaultHoldings {
    fn from(val: &UserVaultHoldings) -> Self {
        DbUserVaultHoldings {
            timestamp: val.timestamp,
            price_per_share: val.price_per_share.to_string(),
            usd_1: val.usd_1.to_string(),
            usd_2: val.usd_2.to_string(),
            usd_all: val.usd_all.to_string(),
        }
    }
}

impl From<&DbUserVaultHoldings> for UserVaultHoldings {
    fn from(val: &DbUserVaultHoldings) -> Self {
        UserVaultHoldings {
            timestamp: val.timestamp,
            price_per_share: BigDecimal::from_str(&val.price_per_share).unwrap(),
            usd_1: BigDecimal::from_str(&val.usd_1).unwrap(),
            usd_2: BigDecimal::from_str(&val.usd_2).unwrap(),
            usd_all: BigDecimal::from_str(&val.usd_all).unwrap(),
        }
    }
}
