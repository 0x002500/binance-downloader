use serde::Serialize;

// Struct for writing to CSV
#[derive(Debug, Serialize)]
pub struct Kline {
    #[serde(rename = "Open Time")]
    pub open_time: String,
    #[serde(rename = "Open")]
    pub open: String,
    #[serde(rename = "High")]
    pub high: String,
    #[serde(rename = "Low")]
    pub low: String,
    #[serde(rename = "Close")]
    pub close: String,
    #[serde(rename = "Volume")]
    pub volume: String,
    #[serde(rename = "Close Time")]
    pub close_time: String,
    #[serde(rename = "Quote Asset Volume")]
    pub quote_asset_volume: String,
    #[serde(rename = "Number of Trades")]
    pub number_of_trades: i64,
    #[serde(rename = "Taker Buy Base Asset Volume")]
    pub taker_buy_base_asset_volume: String,
    #[serde(rename = "Taker Buy Quote Asset Volume")]
    pub taker_buy_quote_asset_volume: String,
    #[serde(rename = "Ignore")]
    pub ignore: String,
}
