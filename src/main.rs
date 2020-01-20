extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Any, Child, Class};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://open.kattis.com/problems/sequences")?;
    println!("{:#?}", resp);

    let document = Document::from_read(resp).unwrap();

    let mut content_check: i8 = 0;

    let mut info: String = "".to_string();
    let mut input: String = "".to_string();
    let mut output: String = "".to_string();

    for node in document.find(Child(Class("problembody"), Any)) {
        // empty lines are nodes for some reason.
        match node.name() {
            None => continue,
            Some("p") => {
                println!("text_test: {}", node.text());
                match content_check {
                    0 => info.push_str(&node.text()),
                    1 => input.push_str(&node.text()),
                    2 => output.push_str(&node.text()),
                    _ => panic!("content_check is exceeding expected number 2."),
                }
            }
            Some("h2") => content_check += 1,
            Some("table") => println!("fångar table"),
            Some(&_) => (),
        };
    }

    println!("INFO: \n\n{:?}", info); // replace "\n....." with " "

    Ok(())
}
