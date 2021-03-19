use bigdecimal::{BigDecimal, FromPrimitive};
use ethers::types::U256;
use std::str::FromStr;
use std::time::SystemTime;

pub trait ToBigDecimal {
    fn to_big_dec(&self) -> BigDecimal;
}

impl ToBigDecimal for U256 {
    fn to_big_dec(&self) -> BigDecimal {
        BigDecimal::from_str(&self.to_string()).expect("Not a valid number")
    }
}

pub trait Scale {
    // https://www.reddit.com/r/Compound/comments/ezk9i4/trying_to_make_sense_of_the_exchange_rate/
    fn scale_1e8(&self) -> BigDecimal;

    fn scale_1e18(&self) -> BigDecimal;
}

impl Scale for BigDecimal {
    fn scale_1e8(&self) -> BigDecimal {
        self / BigDecimal::from_f64(1e8).unwrap()
    }

    fn scale_1e18(&self) -> BigDecimal {
        self / BigDecimal::from_f64(1e18).unwrap()
    }
}

/// Given some assets, a total number of shares and a given number of my shares, calculate
/// the number of assets I onw based on my number of shares.
pub fn scale_to_share(
    assets: &BigDecimal,
    total_shares: &BigDecimal,
    my_shares: &BigDecimal,
) -> BigDecimal {
    assets / total_shares * my_shares
}

/// Return seconds since UNIX epoch
pub fn unix_time() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
