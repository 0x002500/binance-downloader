use serde::Deserialize;

// Define a struct to hold the Kline data with serde for JSON deserialization
// use a Vec<serde_json::Value> here to handle the dynamic nature of the API's array response.
#[derive(Debug, Deserialize)]
pub struct KlineData(
    f64,    // Open time
    String, // Open price
    String, // High price
    String, // Low price
    String, // Close price
    String, // Volume
    f64,    // Close time
    String, // Quote asset volume
    i64,    // Number of trades
    String, // Taker buy base asset volume
    String, // Taker buy quote asset volume
    String, // Ignore
);
