extern crate reqwest;
extern crate select;

mod app {

    use std::env;
    use std::error::Error;

    pub fn run_app() -> Result<(), Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();

        if args.len() == 1 {
            println!("Please use a command.",);
            return Ok(());
        }

        match args[1].as_str() {
            "init" => init::init(),
            _ => println!("Valid commands include: init",),
        }

        Ok(())
    }

    mod init {
        use chrono::{DateTime, Utc};
        use reqwest::StatusCode;
        use select::document::Document;
        use select::predicate::{And, Any, Child, Class, Element, Name};
        use std::fs;
        use std::io::Write;

        #[derive(Default)]
        struct KattisData {
            description: String,
            input_description: String,
            output_description: String,
            tests: std::vec::Vec<(String, String)>,
        }

        fn prompt(prompt: String) -> String {
            print!("{}: ", prompt);
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(n) => {
                    println!("{} bytes read", n);
                    println!("{}", input);
                    input.trim().to_string()
                }
                Err(e) => {
                    println!("error: {}", e);
                    "".to_string()
                }
            }
        }

        fn make_header(data: &KattisData) -> String {
            let author: String = prompt("Author: ".to_string());
            let now: DateTime<Utc> = Utc::now();
            let input = &data.input_description;
            let output = &data.output_description;

            format!(
                "#author: {author}\n#date: {now}\n\n#Input: {input}\n\n#Output: {output}\n\n",
                author = author,
                now = now,
                input = input,
                output = output
            )
        }

        fn write_main_file(content: String, language: String) -> Result<(), std::io::Error> {
            fn file_ending(language: String) -> String {
                match language.to_lowercase().as_ref() {
                    "python" => ".py".to_string(),
                    _ => "".to_string(),
                }
            }
            let file_ending: String = file_ending(language);
            let file_name: String = format!("main{}", file_ending);
            fs::write(file_name, content)
        }

        fn write_test_files(tests: &std::vec::Vec<(String, String)>) -> Result<(), std::io::Error> {
            for (i, (input, output)) in tests.iter().enumerate() {
                fs::write(format!("test{}.in", i), input)?;
                fs::write(format!("test{}.out", i), output)?;
            }

            Ok(())
        }

        pub fn init() {
            let initialization = || -> Result<(), std::io::Error> {
                let url = prompt("Kattis URL: ".to_string());
                let language = prompt("Language: ".to_string());
                let document = fetch_document(url).unwrap();
                let data = parse_document(document).unwrap();
                let header: String = make_header(&data);
                write_main_file(header, language)?;
                write_test_files(&data.tests)?;
                Ok(())
            };

            if let Err(_err) = initialization() {
                panic!("Failed to initiate Kattis files.",);
            }
        }

        fn fetch_document(url: String) -> Result<Document, Box<dyn std::error::Error>> {
            let resp = reqwest::blocking::get(&url)?;

            match resp.status() {
                StatusCode::OK => Ok(Document::from_read(resp).unwrap()),
                _ => panic!("Kattis is unavailable at the moment."), // TODO: return and handle error instead of killing process.
            }
        }

        fn parse_document(document: Document) -> Result<KattisData, std::io::Error> {
            let mut content_check: i8 = 0;
            let mut parsed_data = KattisData::default();

            for node in document.find(And(Element, Child(Class("problembody"), Any))) {
                match node.name() {
                    Some("p") => {
                        let text = node.text().trim().replace("\n   ", "").replace("$", "");
                        match content_check {
                            0 => parsed_data.description.push_str(&text),
                            1 => parsed_data.input_description.push_str(&text),
                            2 => parsed_data.output_description.push_str(&text),
                            _ => panic!("Encountered trouble parsing data: content_check > 2."), // TODO: make this an error.
                        }
                    }
                    Some("h2") => content_check += 1,
                    Some("table") => {
                        parsed_data.tests.push(parse_table(node));
                    }
                    Some(&_) | None => continue,
                };
            }
            Ok(parsed_data)
        }

        fn parse_table(table: select::node::Node) -> (String, String) {
            let res: Vec<String> = table
                .find(And(Element, Name("pre")))
                .map(|n| n.inner_html())
                .collect();

            (res[0].to_string(), res[1].to_string())
        }
    }
}

fn main() {
    std::process::exit(match app::run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
