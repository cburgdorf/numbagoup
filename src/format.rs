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

pub fn print_result(current_holdings: &UserVaultHoldings, performance: &VaultPerformance) {
    print!(
"
crvCOMP     |{:10.4}   |{:11.2}|{:12.2}|{:8.2} ({:5.2} %)|{:8.2} ({:5.2} %)|{:8.2} ({:5.2} %)|{:8.2} ({:5.2} %)|
------------|-------------|-----------|------------|------------------|------------------|------------------|------------------|
",
current_holdings.price_per_share,
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
