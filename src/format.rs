use bigdecimal::BigDecimal;

use crate::types::{UserVaultHoldings, VaultPerformance};

pub fn print_header() {
    print!(
"
--------------------------------------------------------------------------------------------------------------------------------
            |             |           |                                    🚜 Gain (USD) / APY 📈                              |
------------|-------------|-----------|------------|------------------|------------------|------------------|------------------|
VAULT       |Price / share| USD value | last check |     past hour    |     past day     |     past week    |    past month    |
------------|-------------|-----------|------------|------------------|------------------|------------------|------------------|
"
  );
}

pub fn print_result(
    vault_name: &str,
    current_holdings: &UserVaultHoldings,
    performance: &VaultPerformance,
) {
    print!(
"
{:12}|{:10.4}   |{:11.2}|{:12.2}|{:8.2} ({:5.2} %)|{:8.2} ({:5.2} %)|{:8.2} ({:5.2} %)|{:8.2} ({:5.2} %)|
------------|-------------|-----------|------------|------------------|------------------|------------------|------------------|
",
vault_name,
current_holdings.price_per_share,
current_holdings.usd_all,
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

pub fn print_footer(total: BigDecimal, performance: &VaultPerformance) {
    print!(
"
TOTAL       |             |{:11.2}|{:12.2}|{:8.2}          |{:8.2}          |{:8.2}          |{:8.2}          |
------------|-------------|-----------|------------|------------------|------------------|------------------|------------------|
",

total,
performance.gain_last_check,
performance.gain_past_hour,
performance.gain_past_day,
performance.gain_past_week,
performance.gain_past_month,
);
}
