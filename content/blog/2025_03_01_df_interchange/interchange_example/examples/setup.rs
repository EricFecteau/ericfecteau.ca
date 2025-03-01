use duckdb::params;
use duckdb::{Connection, DatabaseName};
use polars::prelude::*;
use postgres::Client;
use std::io::{Read, Write};

fn main() {
    // Read Penguins CSV
    let lf = LazyCsvReader::new("./data/penguins.csv")
        .with_has_header(true)
        .finish()
        .unwrap();

    let lf = lf
        .select([col("species"), col("flipper_length_mm"), col("body_mass_g")])
        .filter(col("flipper_length_mm").is_not_null())
        .filter(col("body_mass_g").is_not_null())
        .with_column(col("flipper_length_mm").cast(DataType::Int64))
        .with_column(col("body_mass_g").cast(DataType::Int64))
        .with_row_index("index", None);

    let mut df1 = lf
        .clone()
        .filter(col("index").lt(lit(111)))
        .collect()
        .unwrap()
        .drop("index")
        .unwrap();
    let df2 = lf
        .clone()
        .filter(col("index").gt_eq(lit(111)).and(col("index").lt(lit(222))))
        .collect()
        .unwrap()
        .drop("index")
        .unwrap();
    let mut df3 = lf
        .clone()
        .filter(col("index").gt_eq(lit(222)))
        .collect()
        .unwrap()
        .drop("index")
        .unwrap();

    // Parquet
    let mut file = std::fs::File::create("./data/penguins.parquet").unwrap();
    ParquetWriter::new(&mut file).finish(&mut df1).unwrap();

    // DuckDB
    let _ = std::fs::remove_file("./data/penguins.duckdb");
    let conn = Connection::open("./data/penguins.duckdb").unwrap();
    conn.execute(
        "CREATE TABLE penguins (species VARCHAR, flipper_length_mm BIGINT, body_mass_g BIGINT)",
        [],
    )
    .unwrap();
    let mut apppender = conn
        .appender_to_db("penguins", &DatabaseName::Main.to_string())
        .unwrap();
    for r in 0..df2.shape().0 {
        let row = df2.get_row(r).unwrap();
        apppender
            .append_row(params![
                row.0[0].get_str().unwrap(),
                row.0[1].try_extract::<i64>().unwrap(),
                row.0[2].try_extract::<i64>().unwrap(),
            ])
            .unwrap();
    }

    // Postgres
    let mut client = Client::connect("host=localhost user=postgres", postgres::NoTls).unwrap();
    let _ = client.batch_execute("drop TABLE penguins;");
    let mut ct_string = String::new();
    ct_string.push_str(
        "CREATE TABLE penguins (species varchar, flipper_length_mm integer, body_mass_g integer)",
    );
    client.batch_execute(&ct_string).unwrap();
    let mut file = std::fs::File::create("./data/p3.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df3).unwrap();
    let mut f = std::fs::File::open("./data/p3.csv").unwrap();
    let metadata = std::fs::metadata("./data/p3.csv").unwrap();
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).unwrap();

    let mut writer = client
        .copy_in("COPY penguins FROM STDIN CSV HEADER")
        .unwrap();
    writer.write_all(&buffer).unwrap();
    writer.finish().unwrap();
    let _ = std::fs::remove_file("./data/p3.csv");
}
