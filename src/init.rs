pub mod init {
    use chrono::{DateTime, Utc};
    use directories::BaseDirs;
    use reqwest::StatusCode;
    use select::document::Document;
    use select::predicate::{And, Any, Child, Class, Element, Name};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::io::{stdin, stdout, Write};
    use std::{env, fs};

    use config::Config;
    use std::error::Error;
    use std::sync::RwLock;

    static FOLDER_NAME: &str = "Mjau"; // duplicate, PLEASE FIX. [init, setup]
                                       // #[derive(Serialize, Deserialize, Debug)]
                                       // struct Config {
                                       //     author: String,
                                       //     email: String,
                                       //     default_language: String,
                                       // }

    pub fn test_settings() {
        let test = super::get_settings();
        println!("VALUE: {}", test);
        // super::main::try_settings();
        // println!("init {}", SETTINGS.read()?.get::<i32>("property")?);
    }

    // fn fetch_default_language() -> Option<String> {
    //     if let Some(base_dirs) = BaseDirs::new() {
    //         let path = base_dirs.data_local_dir().join(FOLDER_NAME);
    //         match fs::read_to_string(path.join("settings.toml")) {
    //             Ok(v) => {
    //                 let config: Config = toml::de::from_str(&v).unwrap();
    //                 return Some(config.default_language);
    //             }
    //             Err(e) => return None,
    //         };
    //     };
    //     None
    // }

    // pub fn confirm_language() -> String {
    //     let empty_string = "".to_string();
    //     let default_language: String = match fetch_default_language() {
    //         None => {
    //             println!("No default language could be found.",);
    //             empty_string
    //         }
    //         Some(v) => {
    //             println!("Language found: {}", v);
    //             v
    //         }
    //     };

    //     println!("default language {}", default_language);

    //     default_language
    // }

    // // duplicate!
    // fn prompt(prompt: String) -> String {
    //     print!("{}: ", prompt);
    //     std::io::stdout().flush().unwrap();

    //     let mut input = String::new();
    //     match std::io::stdin().read_line(&mut input) {
    //         Ok(2) => "".to_string(),
    //         Ok(n) => {
    //             println!("{} bytes read", n);
    //             println!("{}", input);
    //             input
    //         }
    //         Err(e) => {
    //             println!("error: {}", e);
    //             "".to_string()
    //         }
    //     }
    // }

    // pub fn init() {
    //     let kattis_data = || -> Result<(HashMap<String, String>, std::vec::Vec<(String, String)>), std::io::Error> {
    //     let url = prompt("Kattis URL: ".to_string());
    //     let document = fetch_document(url).unwrap();
    //     let (texts, tests) = parse_document(document);
    //         Ok((texts, tests))
    //     };

    //     if let Err(_err) = kattis_data() {
    //         println!("Failed to retrieve Kattis data.", );
    //         return
    //     }

    //     //     texts: HashMap<String, String>,
    //     //     tests: std::vec::Vec<(String, String)>,
    //     // ) -> Result<(), std::io::Error> {
    //     //     let author: String = "Caroline Borg".to_string();
    //     //     let now: DateTime<Utc> = Utc::now();

    //     //     let header: String = format!(
    //     //         "#author:{author}\n#date:{now}\n\n#Input: {input}\n\n#Output: {output}\n\n",
    //     //         author = author,
    //     //         now = now,
    //     //         input = texts.get("input").unwrap_or(&"".to_string()),
    //     //         output = texts.get("output").unwrap_or(&"".to_string())
    //     //     );

    //     //     fs::write("main.py", header)?;

    //     //     Ok(())
    // }

    // fn fetch_document(url: String) -> Result<Document, Box<dyn std::error::Error>> {
    //     let resp = reqwest::blocking::get(&url)?;

    //     match resp.status() {
    //         StatusCode::OK => println!("OK!",),
    //         s => {
    //             println!(
    //                 "Received response status {:?}, program will not continue.",
    //                 s
    //             );
    //             std::process::exit(0);
    //         }
    //     };

    //     Ok(Document::from_read(resp).unwrap())
    // }

    // fn parse_document(
    //     document: Document,
    // ) -> (HashMap<String, String>, std::vec::Vec<(String, String)>) {
    //     let mut content_check: i8 = 0;

    //     let mut info: String = "".to_string();
    //     let mut input: String = "".to_string();
    //     let mut output: String = "".to_string();
    //     let mut tests: std::vec::Vec<(String, String)> = Vec::new();

    //     for node in document.find(And(Element, Child(Class("problembody"), Any))) {
    //         match node.name() {
    //             Some("p") => match content_check {
    //                 0 => info.push_str(&node.text()),
    //                 1 => input.push_str(&node.text()),
    //                 2 => output.push_str(&node.text()),
    //                 _ => panic!("content_check is exceeding expected number 2."),
    //             },
    //             Some("h2") => content_check += 1,
    //             Some("table") => {
    //                 tests.push(parse_table(node));
    //             }
    //             Some(&_) => continue,
    //             None => continue,
    //         };
    //     }

    //     let mut hmap = HashMap::new();
    //     hmap.insert("info".to_string(), info);
    //     hmap.insert("input".to_string(), input);
    //     hmap.insert("output".to_string(), output);
    //     (hmap, tests)
    // }

    // fn parse_table(table: select::node::Node) -> (String, String) {
    //     let res: Vec<String> = table
    //         .find(And(Element, Name("pre")))
    //         .map(|n| n.inner_html())
    //         .collect();

    //     (res[0].to_string(), res[1].to_string())
    // }
}
