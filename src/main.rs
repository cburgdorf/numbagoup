use anyhow::Result;
use clap::{App, Arg};
use ethers::prelude::*;
use std::convert::TryFrom;
use std::str::FromStr;

mod constants;
mod contracts;
mod utils;

use crate::constants::*;
use crate::contracts::{CurveCompLPToken, CurveRegistry, YearnVaultV1};
use crate::utils::{scale_to_share, Scale, ToBigDecimal};

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

    let me = Address::from_str(holder_address).expect("Holder address is invalid");

    let yearn_cmp_vault =
        YearnVaultV1::new(&provider, YEARN_VAULT_V1_ABI, YEARN_CRV_COMP_VAULT_ADDRESS);

    let curve_registry = CurveRegistry::new(&provider, CURVE_REGISTRY_ABI, CURVE_REGISTRY_ADDRESS);
    let curve_comp_lp_token =
        CurveCompLPToken::new(&provider, CURVE_COMP_LP_TOKEN_ABI, CURVE_LP_TOKEN_ADDRESS);

    // Get the DAI+USDC holdings of the Curve Comp Pool
    let balances = curve_registry.get_comp_dai_usdc().await?;
    let dai_in_curve = balances.get(0).unwrap().to_big_dec();
    let usdc_in_curve = balances.get(1).unwrap().to_big_dec();

    // Also read the pure CDAI+CUSDC balances from the pool
    let cbalances = curve_registry.get_comp_cdai_cusdc().await?;
    let cdai_in_curve = cbalances.get(0).unwrap().to_big_dec();
    let cusdc_in_curve = cbalances.get(1).unwrap().to_big_dec();

    // Get the total number of LP Tokens for that pool
    let total_lp_tokens = &curve_comp_lp_token.total_supply().await?.to_big_dec();

    // Get the number of yearn vault shares that I own
    let my_vault_shares = &yearn_cmp_vault.balance_of(me).await?.to_big_dec();

    // Get the price per vault share and scale it down by 1e18
    let price_per_share = &yearn_cmp_vault
        .get_price_per_share()
        .await?
        .to_big_dec()
        .scale_1e18();

    // Based on my vault shares and the price per share, calculate my number of LP tokens for the curve pool
    let my_crv_lp_tokens = my_vault_shares * price_per_share;

    // Scale the holdings of the Curve Pool down to the number of my LP tokens
    let my_usdc = scale_to_share(&usdc_in_curve, &total_lp_tokens, &my_crv_lp_tokens);
    let my_dai = scale_to_share(&dai_in_curve, &total_lp_tokens, &my_crv_lp_tokens);
    let my_cusdc = scale_to_share(&cusdc_in_curve, &total_lp_tokens, &my_crv_lp_tokens);
    let my_cdai = scale_to_share(&cdai_in_curve, &total_lp_tokens, &my_crv_lp_tokens);

    // Sum up USDC and DAI (and we assume both are equal to 1 USD)
    let both = &my_usdc + &my_dai;
    let cboth = &my_cusdc + &my_cdai;

    println!("==CRV COMP VAULT PARAMS==");
    println!("Price Per Share {:.4}", price_per_share);
    println!("==MY CRV COMP VAULT Holdings==");
    println!("CDAI {:.4}", &my_cdai.scale_1e8());
    println!("CUSDC {:.4}", &my_cusdc.scale_1e8());
    println!("CUSDC+CDAI {:.4}", &cboth.scale_1e8());
    println!("DAI {:.4}", &my_dai.scale_1e18());
    println!("USDC {:.4}", &my_usdc.scale_1e18());
    println!("USDC+DAI {:.4}", &both.scale_1e18());
    Ok(())
}
