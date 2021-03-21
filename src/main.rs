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
mod format;
mod store;
mod types;
mod utils;
mod vaults;

use crate::calculations::get_performance;
use crate::format::{print_header, print_result};
use crate::store::{init_default_db, read_entries, save_entry};
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

    print_header();
    print_result(&holdings, &performance);

    Ok(())
}
