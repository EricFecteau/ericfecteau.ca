+++
title = "DataFrame Interchange: An Example"
date = "2025-03-01"
description = "Using `Polars`, `Arrow`, `ConnectorX`, `DuckDB`, `Plotlars` and `Hypors` in the same Rust data pipeline."

[taxonomies]
tags = ["data science", "arrow", "polars", "rust"]
+++

Recently I have been trying to explore the data analysis crates available in Rust, with the thesis that "Rust is data analysis ready". Turns out that while there are tons of really great data analysis crates in Rust, mostly based on [Polars](https://docs.rs/polars/latest/polars/), they don't work well as an ecosystem. Because of Rust's strong type system, you can't take the `Polars 0.45` output from one crate and give it to another crate that assumes `Polars 0.43`. It just won't work - you will get a `error[E0308]: mismatched types`. On top of that, many crates only output [arrow-rs](https://docs.rs/arrow/latest/arrow/) data, and the support for `arrow-rs` was [removed in Polars 0.44](https://github.com/pola-rs/polars/pull/19312). All of this was one serious blocker to Rust being data analysis ready! The interoperability for these crates were assumed to take place on the Python side of things, not within Rust.

Since `Polars` uses [Apache Arrow](https://arrow.apache.org/)’s memory model, and the Arrow memory model implements a [C data interchange format](https://arrow.apache.org/docs/format/CDataInterface.html), this makes it so that zero-copy data interchange can be implemented between any version of `Polars` and any version of `Arrow`. This is what my [df-interchange](https://github.com/EricFecteau/df-interchange) crate does! With the correct version of `Arrow` or `Polars` enabled as a feature flag (e.g. `polars_0_41`, `polars_0_46`, `arrow_53`) you can move data between any version of `Polars (>=0.40)` and any version of `Arrow (>=50)` directly within a data pipeline.

## Data pipeline example

Here is a working data pipeline examples that takes data from a `.parquet` with `Polars 0.46`, a `.duckdb` database with [DuckDB](https://docs.rs/duckdb/latest/duckdb/) (returning an `Arrow 53` `RecordBatch` vector) and a `PostgreSQL` database with [ConnectorX](https://docs.rs/connectorx/latest/connectorx/) (returning a `Polars 0.45` `DataFrame`). These files are created using the [Palmer Penguins](https://allisonhorst.github.io/palmerpenguins/) data and the three sources can be seeded with [this Rust script](https://github.com/EricFecteau/ericfecteau.ca/blob/main/content/blog/2025_03_01_df_interchange/interchange_example/examples/setup.rs).

Now that we have the seeded files, we can read them in, concatenate them and pass them to [Plotlars](https://docs.rs/plotlars/latest/plotlars/) for data visualization and to [Hypors](https://docs.rs/hypors/latest/hypors/) for hypothesis testing. The full script can be found [here](https://github.com/EricFecteau/ericfecteau.ca/blob/main/content/blog/2025_03_01_df_interchange/interchange_example/examples/interchange.rs).

Lastly, here are the crates, versions and features for this example:

```toml
[dependencies]
polars = { version = "0.46", features = ["parquet", "pivot", "lazy"] }
connectorx = { version = "0.4.1", features = ["src_postgres", "dst_arrow", "dst_polars"] }
duckdb = "1.1"
hypors = "0.2.5"
plotlars = "0.8.1"
df-interchange = { version = "0.1", features = ["polars_0_43", "polars_0_45", "polars_0_46", "arrow_53"] }
```

### Reading the data

To start, we can read the three part `Penguin` data from the `.parquet` file, the `.duckdb` database and a PostgreSQL database.

Read ~1/3rd of the Penguin data from `.parquet` using `Polars`. Returns a Polars 0.46 `DataFrame`. 

```Rust
let mut file = std::fs::File::open("./data/penguins.parquet").unwrap();
let polars = ParquetReader::new(&mut file).finish().unwrap();
```

Read ~1/3rd of the Penguin data from DuckDB. Returns an Arrow 53 `Vec<RecordBatch>`.

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

### Interchange

So now we have `Polars 0.46`, `Arrow 53` and `Polars 0.45` data in memory. If you try to concatenate the two Polars `DataFrame` you will get a `[E0308]: mismatched types error`. The `df-interchange` crate can be used to convert two of the data objects to `Polars 0.46`. 

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

Next we can convert the `Polars 0.45` `DataFrame` we got from `ConnectorX` to `Polars 0.46` using `Interchange::from_polars_0_45()` and `.to_polars_0_46()`:

```Rust
let connectorx = Interchange::from_polars_0_45(connectorx)
    .unwrap()
    .to_polars_0_46()
    .unwrap()
    .lazy();
```

Now that we have three in-memory data object using the `Polars 0.46` crate, we can concatenate it (using Polars' `LazyFrame`):

```Rust
let polars = concat(
    vec![polars.lazy(), duckdb, connectorx],
    UnionArgs::default(),
)
.unwrap();
```

### Plotlars

Now that we have one concatenated `LazyFrame` in memory called `polars`, we can pass a copy of it to `Plotlars` to create a graphic! `Plotlars` takes `Polars 0.45`, so lets convert it to that with `Interchange::from_polars_0_46()` and `.to_polars_0_45()`:

```Rust
let polars_0_45 = Interchange::from_polars_0_46(polars.clone().collect().unwrap())
    .unwrap()
    .to_polars_0_45()
    .unwrap();
```

And now we can render the graph as html:

```Rust
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

See output [here](plot.html).

### Hypors

Using the same concatenated `LazyFrame` called `polars`, we can modify and pivot it, for it to be accepted by `Hypors` in order to do an Analysis of Variance (ANOVA) Tests.


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

```

Once properly configured, we can convert it to `Polars 0.43`:

```Rust
let polars_pivot = Interchange::from_polars_0_46(polars_pivot)
    .unwrap()
    .to_polars_0_43()
    .unwrap();
```

And now we can pass the columns to the `anova()` function and print the results!

```Rust
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
```

```
F-statistic: 594.8016274385171
p-value: 0
```

## Conclusion

Prior to [df-interchange](https://github.com/EricFecteau/df-interchange), attempting to do this in Rust would have been extremely hard. You would likely have had to read each sources, convert them to Parquet, then re-read them with the correct version of `Polars` for each of the crates (`Plotlars` and `Hypors`). This would require a lot more reading and writing of data, trivial for small tables like this, but in a real world example can make the pipeline incredibly slow. Now it's as simple as adding a few lines of code and passing the correct version of the object to the analysis crates.