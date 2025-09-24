use chrono::{NaiveDate, TimeZone, Utc};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::{Duration, Instant};

// Define a struct to hold the Kline data with serde for JSON deserialization
// We use a Vec<serde_json::Value> here to handle the dynamic nature of the API's array response.
#[derive(Debug, Deserialize)]
struct KlineData(
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

// Struct for writing to CSV
#[derive(Debug, serde::Serialize)]
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

/// Helper function to convert an interval string to milliseconds.
fn get_interval_milliseconds(interval: &str) -> Option<i64> {
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

/// Fetches all Klines for a given range and sends them to a sender.
fn get_all_klines_in_range(
    symbol: &str,
    interval: &str,
    start_date_str: &str,
    end_date_str: &str,
    sender: Sender<Kline>,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let base_url = "https://data-api.binance.vision/api/v3/klines";
    let mut total_klines_count = 0;

    // Convert date strings to UTC timestamps (milliseconds)
    let start_timestamp = Utc
        .from_utc_datetime(
            &NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d")?
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .timestamp_millis();
    let end_timestamp = Utc
        .from_utc_datetime(
            &NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d")?
                .and_hms_opt(23, 59, 59)
                .unwrap(),
        )
        .timestamp_millis();

    // Estimate the total number of klines for the progress bar
    let interval_ms = get_interval_milliseconds(interval).ok_or("Invalid interval string")?;
    let estimated_klines_count = ((end_timestamp - start_timestamp) / interval_ms) as u64;

    let pb = ProgressBar::new(estimated_klines_count);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")?
        .progress_chars("#>-"));
    pb.set_message("Downloading Klines");

    let mut current_start_time = start_timestamp;

    while current_start_time <= end_timestamp {
        let params = [
            ("symbol", symbol),
            ("interval", interval),
            ("limit", "1000"),
            ("startTime", &current_start_time.to_string()),
            ("endTime", &end_timestamp.to_string()),
        ];

        let start_request_time = Instant::now();
        let response = client.get(base_url).query(&params).send()?;

        if response.status().is_success() {
            let klines_data: Vec<KlineData> = response.json()?;

            if klines_data.is_empty() {
                break;
            }

            for kline in klines_data.iter() {
                if kline.0 <= end_timestamp as f64 {
                    sender.send(Kline {
                        open_time: Utc
                            .timestamp_millis_opt(kline.0 as i64)
                            .unwrap()
                            .to_string(),
                        open: kline.1.clone(),
                        high: kline.2.clone(),
                        low: kline.3.clone(),
                        close: kline.4.clone(),
                        volume: kline.5.clone(),
                        close_time: Utc
                            .timestamp_millis_opt(kline.6 as i64)
                            .unwrap()
                            .to_string(),
                        quote_asset_volume: kline.7.clone(),
                        number_of_trades: kline.8,
                        taker_buy_base_asset_volume: kline.9.clone(),
                        taker_buy_quote_asset_volume: kline.10.clone(),
                        ignore: kline.11.clone(),
                    })?;
                    total_klines_count += 1;
                }
            }

            pb.set_position(total_klines_count as u64);

            let last_kline_open_time = klines_data.last().unwrap().0 as i64;
            if last_kline_open_time >= end_timestamp {
                break;
            }
            current_start_time = last_kline_open_time + 1;

            let elapsed_time = start_request_time.elapsed();
            if elapsed_time < Duration::from_secs(1) {
                std::thread::sleep(Duration::from_secs(1) - elapsed_time);
            }
        } else {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }
    }

    pb.finish_with_message("Download complete.");
    Ok(())
}

/// Saves the vector of Klines to a CSV file.
fn save_to_csv(
    receiver: Receiver<Kline>,
    symbol: &str,
    interval: &str,
    start_date_str: &str,
    end_date_str: &str,
) -> Result<(), Box<dyn Error>> {
    let file_name = format!(
        "{}_{}_{}_to_{}.csv",
        symbol, interval, start_date_str, end_date_str
    );
    let file = File::create(&file_name)?;
    let mut wtr = csv::Writer::from_writer(file);

    // Write header
    wtr.write_record(&[
        "Open Time",
        "Open",
        "High",
        "Low",
        "Close",
        "Volume",
        "Close Time",
        "Quote Asset Volume",
        "Number of Trades",
        "Taker Buy Base Asset Volume",
        "Taker Buy Quote Asset Volume",
        "Ignore",
    ])?;

    for kline in receiver {
        wtr.serialize(kline)?;
    }

    wtr.flush()?;
    println!("\n数据已成功保存到文件: {}", file_name);
    Ok(())
}

fn main() {
    let symbol = "BTCUSDT";
    let interval = "1d";
    let start_date = "2018-01-01";
    let end_date = "2024-12-31";

    println!(
        "开始获取 {} 从 {} 到 {} 的K线数据...",
        symbol, start_date, end_date
    );

    let (sender, receiver) = channel();

    // Spawn a thread for writing to the CSV file
    let writer_handle = thread::spawn(move || {
        if let Err(e) = save_to_csv(receiver, symbol, interval, start_date, end_date) {
            eprintln!("保存CSV文件时发生错误: {}", e);
        }
    });

    // Main thread handles downloading and sending data
    if let Err(e) = get_all_klines_in_range(symbol, interval, start_date, end_date, sender) {
        eprintln!("获取K线数据时发生错误: {}", e);
    }

    // Wait for the writer thread to finish
    writer_handle.join().expect("Writer thread failed");
}
