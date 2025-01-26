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

`dcss-api` is an easy to use Rust and Python wrapper for Dungeon Crawl Stone Soup's (DCSS) Webtile websocket API. It supports logging in, starting a game and sending commands during game play.

<a href="https://github.com/EricFecteau/dcss-api"> <img src="dcss_logo.png" alt="DCSS Logo" width="350"/> </a>

#### [Source](https://github.com/EricFecteau/dcss-api) | [Rust](https://crates.io/crates/dcss-api) | [Python](https://pypi.org/project/dcss-api/) {.centered-text}

## Documentation

The documentation for the dcss-api can be found [here](https://docs.rs/dcss-api/latest/dcss_api/index.html). The best way to start is to look at the examples for [Rust](https://github.com/EricFecteau/dcss-api/tree/main/examples) and [Python](https://github.com/EricFecteau/dcss-api/tree/main/python). 

In depth documentation about the DCSS websocket API can also be found [here](https://ericfecteau.github.io/dcss-api-docs/).

## Setup and install

The project's source files can be found on [GitHub](https://github.com/EricFecteau/dcss-api), and can be installed in [Rust](https://crates.io/crates/dcss-api) and in [Python](https://pypi.org/project/dcss-api/).

The API works for both local and public version of DCSS Webtiles. To run on a public server, you must limit the connection to a maximum of one command every 100 milliseconds (i.e. 10 commands per seconds), by setting the speed_ms option to 100 while connecting. Follow any other rules required by the server's owner.

Due to this, it is preferred to run the API against a local version of DCSS Webtile. You can find installation information on the [DCSS Webtiles Server page](https://github.com/crawl/crawl/tree/master/crawl-ref/source/webserver).

A summary (after installing all prerequisites):

```Bash
git clone "https://github.com/crawl/crawl.git"
cd crawl/crawl-ref/source/
git checkout stone_soup-0.29
make WEBTILES=y
python webserver/server.py
```

Note that this API has been verified to work with version 0.29, 0.30 and 0.31 of DCSS.