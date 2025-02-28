+++
title = "DataFrame Interchange"
date = "2025-02-28"
description = "TEST"

[taxonomies]
tags = ["data science", "arrow", "polars", "rust"]
+++

# 

## Data

Read ~1/3rd of the Penguin data from parquet using Polars. Returns a Polars 0.46 `DataFrame`. 

```Rust
let mut file = std::fs::File::open("./data/penguins.parquet").unwrap();
let polars = ParquetReader::new(&mut file).finish().unwrap();
```

Read ~1/3rd of the Penguin data from DuckDB. Returns a Arrow 53 `Vec<RecordBatch>`.

```Rust
let conn = Connection::open("./data/penguins.duckdb").unwrap();
let mut stmt = conn.prepare("SELECT * FROM penguins").unwrap();
let duckdb: Vec<RecordBatch> = stmt.query_arrow([]).unwrap().collect();
```

Read ~1/3rd of the Penguin data from PostgreSQL with ConnectorX. Returns a Polars 0.45 `DataFrame`.

```Rust
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
```

## Interchange

So now we have `Polars 0.46`, `Arrow 53` and `Polars 0.45` data in memory. If you try to concatenate the two Polars `DataFrame` you will get a `[E0308]: mismatched types error` and since [Poalrs 0.44](https://github.com/pola-rs/polars/pull/19312) you can't import arrow-rs data into Polars. The `df-interchange` crate can solve this, by converting the three data objects to the same version of the same crate. 

Lets first convert the Arrow 53 `Vec<RecordBatch>` we got from `DuckDB` to `Polars 0.46` using `Interchange::from_arrow_53()` and `.to_polars_0_46()`:

```Rust
let duckdb = Interchange::from_arrow_53(duckdb)
    .unwrap()
    .to_polars_0_46()
    .unwrap()
    .lazy()
    .with_column(col("body_mass_g").cast(DataType::Int64))
    .with_column(col("flipper_length_mm").cast(DataType::Int64));
```

Next we can convert the Polars 0.45 `DataFrame` we got from `ConnectorX` to `Polars 0.46` using `Interchange::from_polars_0_45()` and `.to_polars_0_46()`:

```Rust
let connectorx = Interchange::from_polars_0_45(connectorx)
    .unwrap()
    .to_polars_0_46()
    .unwrap()
    .lazy();
```

Now that we have three in-memory data object using the `Polars 0.46` crate, we can concatenate it:

```Rust
let polars = concat(
    vec![polars.lazy(), duckdb, connectorx],
    UnionArgs::default(),
)
.unwrap();
```

## Plotlars

```Rust
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
```

## Hypors

```Rust
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

let result = anova(&[&cols[0], &cols[1], &cols[2]], 0.05).unwrap();

println!(
    "\nF-statistic: {}\np-value: {}\n",
    result.test_statistic, result.p_value
);
```