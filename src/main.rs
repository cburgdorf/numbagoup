#[macro_use]
extern crate derivative;

use anyhow::Result;
use bigdecimal::BigDecimal;
use calculations::get_cumulated_performance;
use chrono::NaiveDateTime;
use clap::{App, Arg};
use ethers::prelude::*;
use std::convert::{TryFrom, TryInto};
use types::{UserVaultHoldings, VaultPerformance};

mod calculations;
mod constants;
mod contracts;
mod format;
mod store;
mod types;
mod utils;
mod vaults;

use crate::calculations::get_performance;
use crate::format::{print_footer, print_header, print_result};
use crate::store::{db_info, init_default_db, read_entries, save_entry};
use crate::types::VaultIdentifier;
use crate::vaults::{get_crvcomp_holdings, get_crvsaave_holdings};

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new("NumbaGoUp")
        .about("Track the holdings of your yearn crvCOMP+crvSAAVE vault go up in USD")
        .arg(
            Arg::with_name("holder-address")
                .help("The address of the vault holder")
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
        let comp_id = VaultIdentifier::new(address, "crvCOMP");
        let saave_id = VaultIdentifier::new(address, "crvSAAVE");

        if matches.is_present("db-info") {
            show_db_info(&comp_id.id())?;
            show_db_info(&saave_id.id())?;
        } else {
            performance_report(&comp_id, &saave_id).await?;
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

async fn performance_report(comp_id: &VaultIdentifier, saave_id: &VaultIdentifier) -> Result<()> {
    let holder_address = &comp_id.address;
    let provider = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27",
    )?;

    let comp_holdings = get_crvcomp_holdings(&provider, &holder_address).await?;
    let saave_holdings = get_crvsaave_holdings(&provider, &holder_address).await?;

    print_header();

    let comp_performance = display_holdings(&comp_id, &comp_holdings)?;
    let saave_performance = display_holdings(&saave_id, &saave_holdings)?;

    let (total, total_performance) = get_cumulated_performance(
        &vec![comp_holdings, saave_holdings],
        &vec![comp_performance, saave_performance],
    );
    print_footer(total, &total_performance);
    Ok(())
}

fn display_holdings(
    group_id: &VaultIdentifier,
    holdings: &UserVaultHoldings,
) -> Result<VaultPerformance> {
    let id = &group_id.id();
    let db = init_default_db().map_err(|err| anyhow::anyhow!(err))?;

    let previous_entries = read_entries(&db, id);

    let gain = previous_entries
        .last()
        .map(|previous| &holdings.usd_all - &previous.usd_all)
        .unwrap_or_else(|| BigDecimal::from(0));

    save_entry(&db, id, &holdings)?;

    let latest_entries = read_entries(&db, id);

    let performance = get_performance(gain, &latest_entries);

    print_result(&group_id.vault_name, &holdings, &performance);

    Ok(performance)
}
