#[cfg(test)]
mod rounding_tests {
    use rust_decimal::Decimal;
    use rust_decimal::RoundingStrategy;
    use std::str::FromStr;

    #[test]
    fn test_midpoint_away_from_zero_2dp() {
        let a = Decimal::from_str("1.005").unwrap();
        assert_eq!(a.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero).to_string(), "1.01");
    }

    #[test]
    fn test_midpoint_away_from_zero_zero_dp() {
        let a = Decimal::from_str("2.5").unwrap();
        assert_eq!(a.round_dp_with_strategy(0, RoundingStrategy::MidpointAwayFromZero).to_string(), "3");
    }
}
