use anyhow::Result;
use ethers::{abi::Abi, abi::Uint, prelude::*};

use std::str::FromStr;

use crate::constants::*;

fn new_contract(provider: &Provider<Http>, abi: &str, address: &str) -> Contract<Provider<Http>> {
    let abi: Abi = serde_json::from_str(abi).expect("Can't load ABI");
    let address = address.parse::<Address>().expect("Invalid address");

    Contract::new(address, abi, provider.clone())
}

// struct CToken {
//     contract: Contract<Provider<Http>>,
// }

// impl CToken {
//     pub fn new(provider: &Provider<Http>, abi: &str, address: &str) -> Self {
//         CToken {
//             contract: new_contract(provider, abi, address),
//         }
//     }

//     pub async fn get_exchange_rate(&self) -> Result<Uint, ContractError<Provider<Http>>> {
//         self.contract
//             .method::<_, Uint>("exchangeRateStored", ())?
//             .call()
//             .await
//     }
// }

pub struct CurveRegistry {
    contract: Contract<Provider<Http>>,
}

impl CurveRegistry {
    pub fn new(provider: &Provider<Http>, abi: &str, address: &str) -> Self {
        CurveRegistry {
            contract: new_contract(provider, abi, address),
        }
    }

    pub async fn get_comp_dai_usdc(&self) -> Result<Vec<Uint>, ContractError<Provider<Http>>> {
        // FIXME: Don't hardcode pool
        let address = Address::from_str(CURVE_COMP_POOL_ADDRESS).unwrap();
        self.contract
            .method::<_, Vec<Uint>>("get_underlying_balances", address)?
            .call()
            .await
    }

    pub async fn get_saave_dai_susd(&self) -> Result<Vec<Uint>, ContractError<Provider<Http>>> {
        // FIXME: Don't hardcode pool
        let address = Address::from_str(CURVE_SAAVE_POOL_ADDRESS).unwrap();
        self.contract
            .method::<_, Vec<Uint>>("get_balances", address)?
            .call()
            .await
    }
}

pub struct CurvePoolLpToken {
    contract: Contract<Provider<Http>>,
}

impl CurvePoolLpToken {
    pub fn new(provider: &Provider<Http>, abi: &str, address: &str) -> Self {
        CurvePoolLpToken {
            contract: new_contract(provider, abi, address),
        }
    }

    pub async fn total_supply(&self) -> Result<Uint, ContractError<Provider<Http>>> {
        self.contract
            .method::<_, Uint>("totalSupply", ())?
            .call()
            .await
    }
}

pub struct YearnVaultV1 {
    contract: Contract<Provider<Http>>,
}

impl YearnVaultV1 {
    pub fn new(provider: &Provider<Http>, abi: &str, address: &str) -> Self {
        YearnVaultV1 {
            contract: new_contract(provider, abi, address),
        }
    }

    pub async fn get_price_per_share(&self) -> Result<Uint, ContractError<Provider<Http>>> {
        self.contract
            .method::<_, Uint>("getPricePerFullShare", ())?
            .call()
            .await
    }

    pub async fn balance_of(
        &self,
        address: Address,
    ) -> Result<Uint, ContractError<Provider<Http>>> {
        self.contract
            .method::<_, Uint>("balanceOf", address)?
            .call()
            .await
    }
}

pub struct YearnVaultV2 {
    contract: Contract<Provider<Http>>,
}

impl YearnVaultV2 {
    pub fn new(provider: &Provider<Http>, abi: &str, address: &str) -> Self {
        YearnVaultV2 {
            contract: new_contract(provider, abi, address),
        }
    }

    pub async fn get_price_per_share(&self) -> Result<Uint, ContractError<Provider<Http>>> {
        self.contract
            .method::<_, Uint>("pricePerShare", ())?
            .call()
            .await
    }

    pub async fn balance_of(
        &self,
        address: Address,
    ) -> Result<Uint, ContractError<Provider<Http>>> {
        self.contract
            .method::<_, Uint>("balanceOf", address)?
            .call()
            .await
    }
}
