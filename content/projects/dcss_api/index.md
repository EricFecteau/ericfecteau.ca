+++
title = "DCSS API"
description = "A Rust and Python API for Dungeon Crawl Stone Soup"
weight = 1
template = "page.html"

[taxonomies]
tags = ["Rust", "DCSS"]

[extra]
local_image = "img/dcss_logo.png"
+++

`dcss-api` is an easy to use Rust and Python wrapper for Dungeon Crawl Stone Soup's (DCSS) Webtile websocket API.

<a href="https://github.com/EricFecteau/dcss-api"> <img src="dcss_logo.png" alt="DCSS Logo" width="350"/> </a>

#### [Source](https://github.com/EricFecteau/dcss-api) | [Rust API](https://crates.io/crates/dcss-api) | [Rust Scenario](https://crates.io/crates/dcss-scenario-builder) | [Python](https://pypi.org/project/dcss-api/) {.centered-text}

## Crates

### [dcss-api](https://github.com/EricFecteau/dcss-api/blob/main/dcss-api/)

`dcss-api` is an easy to use Rust wrapper for DCSS Webtile websocket API. It works with version `0.29`, `0.30`, `0.31` or `0.32` of DCSS.

### [dcss-scenario-builder](https://github.com/EricFecteau/dcss-api/blob/main/dcss-api/)

`dcss-scenario-builder` is a crate to build scenarios in DCSS (wizmode) from a yaml file by providing features, items and monsters and mapping them on a tile map. This is great for testing.

### [dcss-api-python](https://github.com/EricFecteau/dcss-api/tree/main/dcss-api-python)

`dcss-api` is an easy to use Python wrapper for DCSS Webtile websocket API, that includes the `dcss-scenario-builder` functionalities. It works with version `0.29`, `0.30`, `0.31` or `0.32` of DCSS.

### [dcss-data](https://github.com/EricFecteau/dcss-api/tree/main/dcss-data) (experimental)

`dcss-data` is a Rust data model for the data received from DCSS Webtile (tiles, monsters, items, menus, etc.). It is currently experimental and the interface will break without notice.

## Docs

Documentation about the DCSS websocket API and the data it provides can also be found [here](https://ericfecteau.ca/dcss/dcss-api-docs/).