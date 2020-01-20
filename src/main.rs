extern crate reqwest;
extern crate select;

use reqwest::StatusCode;
use select::document::Document;
use select::predicate::{And, Any, Child, Class, Element, Name};

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn run_app() -> Result<(), ()> {
    let document =
        fetch_document("https://open.kattis.com/problems/3dprinter".to_string()).unwrap();

    parse_content(document);

    Ok(())
}

fn fetch_document(url: String) -> Result<Document, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(&url)?;

    match resp.status() {
        StatusCode::OK => println!("OK!",),
        s => {
            println!(
                "Received response status {:?}, program will not continue.",
                s
            );
            std::process::exit(0);
        }
    };

    Ok(Document::from_read(resp).unwrap())
}

fn parse_content(document: Document) {
    let mut content_check: i8 = 0;

    let mut info: String = "".to_string();
    let mut input: String = "".to_string();
    let mut output: String = "".to_string();

    for node in document.find(And(Element, Child(Class("problembody"), Any))) {
        match node.name() {
            Some("p") => match content_check {
                0 => info.push_str(&node.text()),
                1 => input.push_str(&node.text()),
                2 => output.push_str(&node.text()),
                _ => panic!("content_check is exceeding expected number 2."),
            },
            Some("h2") => content_check += 1,
            Some("table") => {
                parse_table(node);
            }
            Some(&_) => (),
            None => continue,
        };
    }
}

fn parse_table(table: select::node::Node) -> (String, String) {
    let res: Vec<String> = table
        .find(And(Element, Name("pre")))
        .map(|n| n.inner_html())
        .collect();

    (res[0].to_string(), res[1].to_string())
}
