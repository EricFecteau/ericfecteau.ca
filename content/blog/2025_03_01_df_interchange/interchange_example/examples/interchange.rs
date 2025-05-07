use connectorx::prelude::*;
use df_interchange::Interchange;
use duckdb::arrow::record_batch::RecordBatch;
use duckdb::Connection;
use hypors::anova::anova;
use plotlars::{Plot, Rgb, ScatterPlot};
use polars::prelude::*;

fn main() {
    // Read ~1/3 Penguin from Parquet with Polars (Polars 0.46)
    let mut file = std::fs::File::open("./data/penguins.parquet").unwrap();
    let polars = ParquetReader::new(&mut file).finish().unwrap();

    // Read ~1/3 from DuckDB with DuckDB (Arrow 53)
    let conn = Connection::open("./data/penguins.duckdb").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM penguins").unwrap();
    let duckdb: Vec<RecordBatch> = stmt.query_arrow([]).unwrap().collect();

    // Read ~1/3 from PostgreSQL with ConnectorX (Polars 0.45)
    let source_conn =
        SourceConn::try_from("postgresql://postgres:postgres@localhost:5432").unwrap();
    let connectorx = get_arrow(
        &source_conn,
        None,
        &[CXQuery::from("SELECT * FROM penguins")],
    )
    .unwrap()
    .polars()
    .unwrap();

    // Concat the data (Polars 0.46)
    let duckdb = Interchange::from_arrow_53(duckdb)
        .unwrap()
        .to_polars_0_46()
        .unwrap()
        .lazy()
        .with_column(col("body_mass_g").cast(DataType::Int64))
        .with_column(col("flipper_length_mm").cast(DataType::Int64));

    let connectorx = Interchange::from_polars_0_45(connectorx)
        .unwrap()
        .to_polars_0_46()
        .unwrap()
        .lazy();

    let polars = concat(
        vec![polars.lazy(), duckdb, connectorx],
        UnionArgs::default(),
    )
    .unwrap();

    // Plot the data with Plotlars (Polars 0.45)
    let polars_0_45 = Interchange::from_polars_0_46(polars.clone().collect().unwrap())
        .unwrap()
        .to_polars_0_45()
        .unwrap();
    let html = ScatterPlot::builder()
        .data(&polars_0_45)
        .x("body_mass_g")
        .y("flipper_length_mm")
        .group("species")
        .opacity(0.5)
        .size(12)
        .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
        .plot_title("Penguin Flipper Length vs Body Mass")
        .x_title("Body Mass (g)")
        .y_title("Flipper Length (mm)")
        .legend_title("Species")
        .build()
        .to_html();
    let mut file = std::fs::File::create("./plot.html").unwrap();
    std::io::Write::write_all(&mut file, html.as_bytes()).unwrap();

    // Hypothesis testing with Hypors (Polars 0.43)
    let polars = polars
        .select([
            col("species"),
            col("flipper_length_mm").cast(DataType::Float64),
        ])
        .with_row_index("index", None);
    let polars_pivot = pivot::pivot_stable(
        &polars.collect().unwrap(),
        ["species"],
        Some(["index"]),
        Some(["flipper_length_mm"]),
        false,
        None,
        None,
    )
    .unwrap()
    .drop("index")
    .unwrap();
    let polars_pivot = Interchange::from_polars_0_46(polars_pivot)
        .unwrap()
        .to_polars_0_43()
        .unwrap();
    let cols = polars_pivot.get_columns();
    let result = anova(
        &[
            &cols[0].drop_nulls(),
            &cols[1].drop_nulls(),
            &cols[2].drop_nulls(),
        ],
        0.05,
    )
    .unwrap();
    println!(
        "\nF-statistic: {}\np-value: {}\n",
        result.test_statistic, result.p_value
    );
}
