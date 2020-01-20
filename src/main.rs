extern crate reqwest;
extern crate select;

use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use select::document::Document;
use select::predicate::{And, Any, Child, Class, Element, Name};
use std::collections::HashMap;
use std::fs;

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

    let (texts, tests) = parse_content(document);

    match setup(texts, tests) {
        Ok(_) => (),
        Err(_) => (),
    }

    Ok(())
}

fn setup(
    texts: HashMap<String, String>,
    tests: std::vec::Vec<(String, String)>,
) -> Result<(), std::io::Error> {
    let author: String = "Caroline Borg".to_string();
    let now: DateTime<Utc> = Utc::now();

    let header: String = format!(
        "#author:{author}\n#date:{now}\n\n#Input: {input}\n\n#Output: {output}\n\n",
        author = author,
        now = now,
        input = texts.get("input").unwrap_or(&"".to_string()),
        output = texts.get("output").unwrap_or(&"".to_string())
    );

    fs::write("main.py", header)?;

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

fn parse_content(document: Document) -> (HashMap<String, String>, std::vec::Vec<(String, String)>) {
    let mut content_check: i8 = 0;

    let mut info: String = "".to_string();
    let mut input: String = "".to_string();
    let mut output: String = "".to_string();
    let mut tests: std::vec::Vec<(String, String)> = Vec::new();

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
                tests.push(parse_table(node));
            }
            Some(&_) => continue,
            None => continue,
        };
    }

    let mut hmap = HashMap::new();
    hmap.insert("info".to_string(), info);
    hmap.insert("input".to_string(), input);
    hmap.insert("output".to_string(), output);
    (hmap, tests)
}

fn parse_table(table: select::node::Node) -> (String, String) {
    let res: Vec<String> = table
        .find(And(Element, Name("pre")))
        .map(|n| n.inner_html())
        .collect();

    (res[0].to_string(), res[1].to_string())
}
