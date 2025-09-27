# Binance Downloader

> An efficient Rust-based tool for downloading historical kline data from Binance.

## Features

- Download historical kline data for any trading pair
- Multi-threaded processing for concurrent download and save operations
- Automatic API rate limit handling
- Support for multiple timeframes (from 1 second to 1 month)
- Progress bar visualization
- Data saved in CSV format for easy analysis

## Usage

1. Build the project:
```bash
cargo build --release
```

2. Run the program:
```bash
./target/release/binance-downloader
```

## Supported Timeframes

- 1s: 1 second
- 1m, 3m, 5m, 15m, 30m: Minute intervals
- 1h, 2h, 4h, 6h, 8h, 12h: Hour intervals
- 1d, 3d: Daily intervals
- 1w: Weekly interval
- 1M: Monthly interval

## Configuration

Modify the following parameters in `main.rs`:

```rust
let symbol = "BTCUSDT";     // Trading pair
let interval = "1d";        // Timeframe
let start_date = "2018-01-01";  // Start date
let end_date = "2024-12-31";    // End date
```

## Output Files

The program generates CSV files with the naming format: `{symbol}_{interval}_{start_date}_to_{end_date}.csv`

## Dependencies

- chrono: Time handling
- csv: CSV file operations
- indicatif: Progress bar display
- reqwest: HTTP requests
- serde: Data serialization
- serde_json: JSON processing

## Important Notes

- Please respect Binance API rate limits
- Use release mode compilation for optimal performance
