pub fn get_interval_milliseconds(interval: &str) -> Option<i64> {
    match interval {
        "1s" => Some(1000),
        "1m" => Some(60_000),
        "3m" => Some(180_000),
        "5m" => Some(300_000),
        "15m" => Some(900_000),
        "30m" => Some(1_800_000),
        "1h" => Some(3_600_000),
        "2h" => Some(7_200_000),
        "4h" => Some(14_400_000),
        "6h" => Some(21_600_000),
        "8h" => Some(28_800_000),
        "12h" => Some(43_200_000),
        "1d" => Some(86_400_000),
        "3d" => Some(259_200_000),
        "1w" => Some(604_800_000),
        "1M" => Some(2_592_000_000),
        _ => None,
    }
}
