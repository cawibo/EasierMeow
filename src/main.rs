extern crate reqwest;
extern crate select;
extern crate toml;

use chrono::{DateTime, Utc};
use directories::BaseDirs;
use reqwest::StatusCode;
use select::document::Document;
use select::predicate::{And, Any, Child, Class, Element, Name};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::{env, fs};

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
    // let args: Vec<String> = env::args().collect();

    // let command = &args[1];

    // match command.as_str() {
    //     "setup" => setup(),
    //     //"init" => init(),
    //     _ => setup(),
    // }

    setup();

    // let document =
    //     fetch_document("https://open.kattis.com/problems/3dprinter".to_string()).unwrap();

    // let (texts, tests) = parse_content(document);

    // match init(texts, tests) {
    //     Ok(_) => (),
    //     Err(_) => (),
    // }

    Ok(())
}

static FOLDER_NAME: &str = "Mjau";

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    author: String,
    email: String,
    default_language: String,
}

fn setup() -> Result<(), std::io::Error> {
    if let Some(base_dirs) = BaseDirs::new() {
        let path = base_dirs.data_local_dir().join(FOLDER_NAME);
        match local_dir_exists_or_handle(&path) {
            Ok(v) => println!("Local directory {}.", v),
            Err(e) => return Err(e),
        };
        match local_file_exists_and_or_handle(&path) {
            Ok(v) => println!("Local settings {}.", v),
            Err(e) => return Err(e),
        }
    };

    Ok(())
}

fn prompt(prompt: String) -> String {
    print!("{}: ", prompt);
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(2) => "".to_string(),
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
            input
        }
        Err(e) => {
            println!("error: {}", e);
            "".to_string()
        }
    }
}

fn local_file_exists_and_or_handle(path: &std::path::Path) -> Result<String, std::io::Error> {
    let mut action_taken: String = "found".to_string();

    let path_to_settings = path.join("settings.toml");
    if !path_to_settings.exists() {
        let struct_config = Config {
            author: prompt("Name: ".to_string()),
            email: prompt("Email: ".to_string()),
            default_language: prompt("Language: ".to_string()),
        };

        let str_config: String = toml::to_string_pretty(&struct_config).unwrap();
        fs::write(path_to_settings, str_config)?;

        action_taken = "created".to_string();
    };

    Ok(action_taken)
}

fn local_dir_exists_or_handle(path: &std::path::Path) -> Result<String, std::io::Error> {
    let mut action_taken: String = "found".to_string();
    if !path.exists() {
        fs::create_dir(path)?;

        action_taken = "created".to_string();
    };

    Ok(action_taken)
}

fn init(
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
