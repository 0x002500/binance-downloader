use serde::Serialize;

// Struct for writing to CSV
#[derive(Debug, Serialize)]
struct Kline {
    #[serde(rename = "Open Time")]
    open_time: String,
    #[serde(rename = "Open")]
    open: String,
    #[serde(rename = "High")]
    high: String,
    #[serde(rename = "Low")]
    low: String,
    #[serde(rename = "Close")]
    close: String,
    #[serde(rename = "Volume")]
    volume: String,
    #[serde(rename = "Close Time")]
    close_time: String,
    #[serde(rename = "Quote Asset Volume")]
    quote_asset_volume: String,
    #[serde(rename = "Number of Trades")]
    number_of_trades: i64,
    #[serde(rename = "Taker Buy Base Asset Volume")]
    taker_buy_base_asset_volume: String,
    #[serde(rename = "Taker Buy Quote Asset Volume")]
    taker_buy_quote_asset_volume: String,
    #[serde(rename = "Ignore")]
    ignore: String,
}
