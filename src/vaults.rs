use anyhow::Result;
use ethers::prelude::*;
use std::str::FromStr;

use crate::constants::*;
use crate::contracts::{CurvePoolLPToken, CurveRegistry, YearnVaultV1};
use crate::types::UserVaultHoldings;
use crate::utils::{scale_to_share, unix_time, Scale, ToBigDecimal};

pub async fn get_crvcomp_holdings(
    provider: &Provider<Http>,
    holder_address: &str,
) -> Result<UserVaultHoldings> {
    let me = Address::from_str(holder_address).expect("Holder address is invalid");

    let yearn_cmp_vault =
        YearnVaultV1::new(&provider, YEARN_VAULT_V1_ABI, YEARN_CRV_COMP_VAULT_ADDRESS);

    let curve_registry = CurveRegistry::new(&provider, CURVE_REGISTRY_ABI, CURVE_REGISTRY_ADDRESS);
    let curve_comp_lp_token = CurvePoolLPToken::new(
        &provider,
        CURVE_COMP_LP_TOKEN_ABI,
        CURVE_COMP_LP_TOKEN_ADDRESS,
    );

    // Get the DAI+USDC holdings of the Curve Comp Pool
    let balances = curve_registry.get_comp_dai_usdc().await?;
    let dai_in_curve = balances.get(0).unwrap().to_big_dec();
    let usdc_in_curve = balances.get(1).unwrap().to_big_dec();

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

    // Sum up USDC and DAI (and we assume both are equal to 1 USD)
    let both = &my_usdc + &my_dai;

    Ok(UserVaultHoldings {
        timestamp: unix_time(),
        price_per_share: price_per_share.clone(),
        usd_1: my_dai.scale_1e18(),
        usd_2: my_usdc.scale_1e18(),
        usd_all: both.scale_1e18(),
    })
}

pub async fn get_crvsaave_holdings(
    provider: &Provider<Http>,
    holder_address: &str,
) -> Result<UserVaultHoldings> {
    let me = Address::from_str(holder_address).expect("Holder address is invalid");

    let yearn_cmp_vault =
        YearnVaultV1::new(&provider, YEARN_VAULT_V1_ABI, YEARN_CRV_SAAVE_VAULT_ADDRESS);

    let curve_registry = CurveRegistry::new(&provider, CURVE_REGISTRY_ABI, CURVE_REGISTRY_ADDRESS);
    let curve_pool_lp_token = CurvePoolLPToken::new(
        &provider,
        CURVE_SAAVE_LP_TOKEN_ABI,
        CURVE_SAAVE_LP_TOKEN_ADDRESS,
    );

    // Get the DAI+sUSD holdings of the Curve Comp Pool
    let balances = curve_registry.get_saave_dai_susd().await?;
    let dai_in_curve = balances.get(0).unwrap().to_big_dec();
    let susd_in_curve = balances.get(1).unwrap().to_big_dec();

    // Get the total number of LP Tokens for that pool
    let total_lp_tokens = &curve_pool_lp_token.total_supply().await?.to_big_dec();

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
    let my_susd = scale_to_share(&susd_in_curve, &total_lp_tokens, &my_crv_lp_tokens);
    let my_dai = scale_to_share(&dai_in_curve, &total_lp_tokens, &my_crv_lp_tokens);

    // Sum up sUSD and DAI (and we assume both are equal to 1 USD)
    let both = &my_susd + &my_dai;

    Ok(UserVaultHoldings {
        timestamp: unix_time(),
        price_per_share: price_per_share.clone(),
        usd_1: my_dai.scale_1e18(),
        usd_2: my_susd.scale_1e18(),
        usd_all: both.scale_1e18(),
    })
}
