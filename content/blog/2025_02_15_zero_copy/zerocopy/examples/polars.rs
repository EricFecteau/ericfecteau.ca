use polars::prelude::*;
use std::fs::File;
use std::io::prelude::*;

// echo "build" > ./mem_log/prog_type.txt

fn main() {
    let mut file = File::create("./mem_log/prog_type.txt").unwrap();
    file.write_all(b"load").unwrap();

    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(
        "/home/eric/Rust/rust-data-analysis/data/lfs_large/part",
        args,
    )
    .unwrap();

    let years: Vec<i64> = (2006..2018).collect();
    let lf = lf.filter(col("survyear").is_in(lit(Series::from_iter(years))));

    // Need to use it
    println!("{}", lf.collect().unwrap());

    let mut file = File::create("./mem_log/prog_type.txt").unwrap();
    file.write_all(b"done").unwrap();
}
