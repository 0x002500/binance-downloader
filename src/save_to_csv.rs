use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::sync::mpsc::Receiver;

use crate::Kline;

/// Saves the vector of Klines to a CSV file.
pub fn save_to_csv(
    receiver: Receiver<Vec<Kline>>, // Receive batches (Vec<Kline>)
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

    // Use BufWriter to buffer disk writes, reducing system calls.
    let buffered_writer = BufWriter::new(file);
    let mut wtr = csv::Writer::from_writer(buffered_writer);

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

    // Receive batches and serialize all records within each batch
    for kline_batch in receiver {
        for kline in kline_batch {
            wtr.serialize(kline)?;
        }
    }

    // Flush the BufWriter and CSV Writer to ensure all data is written to disk
    wtr.flush()?;
    println!("\n数据已成功保存到文件: {}", file_name);
    Ok(())
}
