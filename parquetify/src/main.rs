use polars::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    parquetify();
    let df = LazyFrame::scan_parquet("test.parquet", Default::default())?.collect()?;
    println!("{:?}", df);

    Ok(())
}

const PATH: &str = "test.csv";

fn parquetify() -> Result<(), Box<dyn std::error::Error>> {
    let df = LazyCsvReader::new(PATH)
        .with_has_header(true)
        // .with_separator(b',')
        .finish()?;
    // take only 100 rows.

    let compression_option = ParquetCompression::Zstd(ZstdLevel::try_new(3).ok());
    let options = ParquetWriteOptions {
        compression: compression_option,
        ..Default::default()
    };
    df.sink_parquet(&"test.parquet", options, None)?;

    Ok(())
}
