# rust-scraping
Examples of web scraping in Rust

This repo is example code for an upcoming article on the [LogRocket blog](https://blog.logrocket.com/). It shows an example of parsing a set of life expectancy pages from the Social Security Administration and emitting a JSON file with the results.

It uses the [reqwest](https://crates.io/crates/reqwest) crate to fetch the pages, the [scraper](https://crates.io/crates/scraper) crate for parsing the pages, and the [json](https://crates.io/crates/json) crate to write the data out.
