use std::{fs::File, io::Write};

use plotlars::{Legend, Line, Plot, Rgb, Shape, Text, TimeSeriesPlot};
use polars::prelude::*;

fn main() {
    let lf = LazyCsvReader::new("./mem_log/out.log")
        .with_has_header(true)
        .with_separator(b' ')
        .finish()
        .unwrap();

    // Limit data
    let lf = lf
        .select([col("time"), col("type"), col("used")])
        .filter(col("type").eq(lit("build")).not());

    // // Convert to time
    // let dt_options = StrptimeOptions {
    //     format: Some("%H:%M:%S%.f".into()),
    //     ..Default::default()
    // };

    // let lf = lf.with_column(
    //     col("time")
    //         .str()
    //         .strptime(DataType::Time, dt_options, lit(""))
    //         .cast(DataType::String),
    // );

    let lf =
        lf.with_column((col("used").cast(DataType::Float64) / (lit(1024) * lit(1024))).round(1));

    println!("{}", lf.clone().collect().unwrap());

    let html = TimeSeriesPlot::builder()
        .data(&lf.collect().unwrap())
        .x("time")
        .y("used")
        .build()
        .to_html();

    let mut file = File::create("./mem_log/plot.html").unwrap();
    file.write_all(html.as_bytes()).unwrap();
}
