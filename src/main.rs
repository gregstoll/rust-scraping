use reqwest::{blocking::Response, Error};

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let body = reqwest::blocking::get("https://www.rust-lang.org")?
        .text()?;

    println!("body = {:?}", body);
    return Ok(());
}
