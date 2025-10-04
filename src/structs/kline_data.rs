use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KlineData(
    pub f64,    // Open time
    pub String, // Open price
    pub String, // High price
    pub String, // Low price
    pub String, // Close price
    pub String, // Volume
    pub f64,    // Close time
    pub String, // Quote asset volume
    pub i64,    // Number of trades
    pub String, // Taker buy base asset volume
    pub String, // Taker buy quote asset volume
    pub String, // Ignore
);
