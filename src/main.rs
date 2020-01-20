extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://open.kattis.com/problems/hello")?;
    println!("{:#?}", resp);

    Document::from_read(resp)?
        .find(Name("div"))
        .for_each(|_x| println!("hittade div"));

    Ok(())
}
