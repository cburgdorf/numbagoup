#[macro_use]
extern crate derivative;

use anyhow::Result;
use bigdecimal::BigDecimal;
use clap::{App, Arg};
use ethers::prelude::*;
use std::convert::TryFrom;

mod calculations;
mod constants;
mod contracts;
mod store;
mod types;
mod utils;
mod vaults;

use crate::calculations::get_performance;

use crate::store::{init_default_db, read_entries, save_entry};
use crate::types::{UserVaultHoldings, VaultPerformance};
use crate::vaults::get_holdings;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("NumbaGoUp")
        .about("Track the holdings of your yearn crvCOMP vault go up in USD")
        .arg(
            Arg::with_name("holder-address")
                .help("The address of the crvCOMP Vault holder")
                .index(1)
                .required(true),
        )
        .get_matches();

    let holder_address = matches.value_of("holder-address").unwrap();

    let provider = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27",
    )?;

    let holdings = get_holdings(provider, holder_address).await?;

    let db = init_default_db().map_err(|err| anyhow::anyhow!(err))?;

    let previous_entries = read_entries(&db);

    let gain = previous_entries
        .last()
        .map(|previous| &holdings.both - &previous.both)
        .unwrap_or_else(|| BigDecimal::from(0));

    save_entry(&db, &holdings)?;

    let latest_entries = read_entries(&db);

    let performance = get_performance(gain, &latest_entries);

    print_result(&holdings, performance);

    Ok(())
}

fn print_result(current_holdings: &UserVaultHoldings, performance: VaultPerformance) {
    print!(
        "
VAULT               |            crvCOMP        |
********************|***************************|
Price per share     |{:14.4}  |          |
--------------------|----------------|          |
CDAI                |{:14.4}  |          |
--------------------|----------------|          |
CUSDC               |{:14.4}  |          |
--------------------|----------------|          |
CUSDC+CDAI          |{:14.4}  |          |
--------------------|----------------|          |
DAI                 |{:14.4}  |          |
--------------------|----------------|          |
USDC                |{:14.4}  |          |
=====================================|          |
USDC + DAI 💰       |{:14.4}  |          |
--------------------|----------------|          |
Gains 🚜 last check |{:14.4}  |   APY    |
--------------------|----------------|==========|
Gains 🚜 past hour  |{:14.4}  | ({:4.1} %) |
--------------------|----------------|----------|
Gains 🚜 past day   |{:14.4}  | ({:4.1} %) |
--------------------|----------------|----------|
Gains 🚜 past week  |{:14.4}  | ({:4.1} %) |
--------------------|----------------|----------|
Gains 🚜 past month |{:14.4}  | ({:4.1} %) |
--------------------|----------------|----------|
    ",
        current_holdings.price_per_share,
        current_holdings.cdai,
        current_holdings.cusdc,
        current_holdings.cboth,
        current_holdings.dai,
        current_holdings.usdc,
        current_holdings.both,
        performance.gain_last_check,
        performance.gain_past_hour,
        performance.apy_past_hour,
        performance.gain_past_day,
        performance.apy_past_day,
        performance.gain_past_week,
        performance.apy_past_week,
        performance.gain_past_month,
        performance.apy_past_month,
    );
}
