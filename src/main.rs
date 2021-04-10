#[macro_use]
extern crate derivative;

use anyhow::Result;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use clap::{App, Arg};
use ethers::prelude::*;
use std::convert::{TryFrom, TryInto};

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
use crate::store::{db_info, init_default_db, read_entries, save_entry};
use crate::types::VaultIdentifier;
use crate::vaults::get_holdings;

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new("NumbaGoUp")
        .about("Track the holdings of your yearn crvCOMP vault go up in USD")
        .arg(
            Arg::with_name("holder-address")
                .help("The address of the crvCOMP Vault holder")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("db-info")
                .long("db-info")
                .takes_value(false)
                .help("Show db info"),
        );

    let matches = app.get_matches();
    if let Some(address) = matches.value_of("holder-address") {
        let vault_id = VaultIdentifier::new(address, "crvCOMP");

        if matches.is_present("db-info") {
            show_db_info(&vault_id.id())?;
        } else {
            performance_report(&vault_id).await?;
        }
    }

    Ok(())
}

fn show_db_info(group_id: &str) -> Result<()> {
    let db = init_default_db().map_err(|err| anyhow::anyhow!(err))?;
    let info = db_info(&db, group_id);
    let oldest = NaiveDateTime::from_timestamp(info.oldest_timestamp.try_into().unwrap(), 0);
    let newest = NaiveDateTime::from_timestamp(info.newest_timestamp.try_into().unwrap(), 0);

    print!(
        "
Total Entries: {}
Oldest: {}
Newest: {}
",
        info.entry_count, oldest, newest
    );
    Ok(())
}

async fn performance_report(vault_identifier: &VaultIdentifier) -> Result<()> {
    let holder_address = &vault_identifier.address;
    let provider = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27",
    )?;

    let holdings = get_holdings(provider, &holder_address).await?;

    let db = init_default_db().map_err(|err| anyhow::anyhow!(err))?;

    let previous_entries = read_entries(&db, &vault_identifier.id());

    let gain = previous_entries
        .last()
        .map(|previous| &holdings.both - &previous.both)
        .unwrap_or_else(|| BigDecimal::from(0));

    save_entry(&db, &vault_identifier.id(), &holdings)?;

    let latest_entries = read_entries(&db, &vault_identifier.id());

    let performance = get_performance(gain, &latest_entries);

    print_header();
    print_result(&holdings, &performance);

    Ok(())
}
