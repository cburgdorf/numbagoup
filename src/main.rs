#[macro_use]
extern crate derivative;

use anyhow::Result;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use clap::{App, Arg, SubCommand};
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
use crate::vaults::get_holdings;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new("NumbaGoUp")
        .about("Track the holdings of your yearn crvCOMP vault go up in USD")
        .subcommand(SubCommand::with_name("db-info"))
        .arg(
            Arg::with_name("holder-address")
                .help("The address of the crvCOMP Vault holder")
                .index(1),
        );

    let matches = app.clone().get_matches();
    if let Some(address) = matches.value_of("holder-address") {
        performance_report(address).await?;
    } else if matches.subcommand_matches("db-info").is_some() {
        show_db_info()?;
    } else {
        app.print_help()?;
    }
    Ok(())
}

fn show_db_info() -> Result<()> {
    let db = init_default_db().map_err(|err| anyhow::anyhow!(err))?;
    let info = db_info(&db);
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

async fn performance_report(holder_address: &str) -> Result<()> {
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
