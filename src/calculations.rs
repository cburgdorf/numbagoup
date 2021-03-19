use crate::constants;
use crate::types::{UserVaultHoldings, VaultPerformance};
use crate::utils::unix_time;
use bigdecimal::BigDecimal;

/// Find the closest entry to the given timestamp
pub fn find_closest_to(entries: &[UserVaultHoldings], timestamp: u64) -> Option<UserVaultHoldings> {
    let mut previous_distance = None;
    let mut previous_entry = None;
    for entry in entries {
        // Distance direction doesn't matter, let's just have it positive
        let distance = if timestamp > entry.timestamp {
            timestamp - entry.timestamp
        } else {
            entry.timestamp - timestamp
        };

        // We assume that entries are sorted so if the previous distance was smaller we know
        // that it won't get better from here. Return the previous entry.
        if matches!(previous_distance, Some(prev) if prev < distance) {
            return previous_entry;
        }

        previous_distance = Some(distance);
        previous_entry = Some(entry.clone());
    }

    previous_entry
}

/// Calculate the gains in the given past duration
pub fn get_gain_in_past_duration(
    entries: &[UserVaultHoldings],
    now: u64,
    duration_sec: u64,
) -> BigDecimal {
    let start_time = now - duration_sec;
    let start_point = find_closest_to(entries, start_time);
    if let (Some(start_holdings), Some(now_holdings)) = (start_point, entries.last()) {
        let gain = &now_holdings.both - &start_holdings.both;
        let actual_duration = now_holdings.timestamp - start_holdings.timestamp;
        if actual_duration == 0 {
            return BigDecimal::from(0);
        }
        let scaled_gain = gain / BigDecimal::from(actual_duration) * BigDecimal::from(duration_sec);
        return scaled_gain;
    }
    BigDecimal::from(0)
}

pub fn get_performance(
    since_last_check: BigDecimal,
    entries: &[UserVaultHoldings],
) -> VaultPerformance {
    VaultPerformance {
        gain_last_check: since_last_check,
        gain_past_hour: get_gain_in_past_duration(entries, unix_time(), constants::HOUR_IN_SEC),
        gain_past_day: get_gain_in_past_duration(entries, unix_time(), constants::DAY_IN_SEC),
        gain_past_week: get_gain_in_past_duration(entries, unix_time(), constants::WEEK_IN_SEC),
        gain_past_month: get_gain_in_past_duration(entries, unix_time(), constants::MONTH_IN_SEC),
    }
}

#[cfg(test)]
mod tests {
    use crate::calculations::{find_closest_to, get_gain_in_past_duration};
    use crate::types::UserVaultHoldings;
    use bigdecimal::BigDecimal;

    impl UserVaultHoldings {
        pub fn with_timestamp(timestamp: u64) -> UserVaultHoldings {
            let mut custom = UserVaultHoldings::zero();
            custom.timestamp = timestamp;
            custom
        }

        pub fn with_timestamp_and_value(timestamp: u64, value: u64) -> UserVaultHoldings {
            let mut custom = UserVaultHoldings::zero();
            custom.timestamp = timestamp;
            custom.both = BigDecimal::from(value);
            custom
        }
    }

    #[test]
    fn test_find_closest_to() {
        let entries: Vec<UserVaultHoldings> = (1..11)
            .map(|num| UserVaultHoldings::with_timestamp(num * 1000))
            .collect();

        assert!(matches!(
            find_closest_to(&entries, 0),
            Some(UserVaultHoldings {
                timestamp: 1000,
                ..
            })
        ));
        assert!(matches!(
            find_closest_to(&entries, 9999),
            Some(UserVaultHoldings {
                timestamp: 10000,
                ..
            })
        ));
        assert!(matches!(
            find_closest_to(&entries, 5000),
            Some(UserVaultHoldings {
                timestamp: 5000,
                ..
            })
        ));
        assert!(matches!(
            find_closest_to(&entries, 4900),
            Some(UserVaultHoldings {
                timestamp: 5000,
                ..
            })
        ));
        assert!(matches!(
            find_closest_to(&entries, 4500),
            Some(UserVaultHoldings {
                timestamp: 5000,
                ..
            })
        ));
        assert!(matches!(
            find_closest_to(&entries, 4499),
            Some(UserVaultHoldings {
                timestamp: 4000,
                ..
            })
        ));
    }

    #[test]
    fn test_get_gain_for_duration() {
        let entries: Vec<UserVaultHoldings> = vec![
            UserVaultHoldings::with_timestamp_and_value(1, 10),
            UserVaultHoldings::with_timestamp_and_value(10, 5),
            UserVaultHoldings::with_timestamp_and_value(20, 20),
            UserVaultHoldings::with_timestamp_and_value(40, 100),
        ];

        let now = 40;

        assert_eq!(
            get_gain_in_past_duration(&entries, now, 20),
            BigDecimal::from(80)
        );
        assert_eq!(
            get_gain_in_past_duration(&entries, now, 15),
            BigDecimal::from(60)
        );
    }
}
