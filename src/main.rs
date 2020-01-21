extern crate reqwest;
extern crate select;
extern crate toml;

#[macro_use]
extern crate lazy_static;

use config::Config;
use std::error::Error;
use std::sync::RwLock;

lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

mod init;
mod setup;

fn get_settings() -> Result<i32, Box<Error>> {
    let val = SETTINGS.read()?.get::<i32>("property")?;

    Ok(val)
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

pub fn try_settings() -> Result<(), Box<Error>> {
    SETTINGS.write()?.set("property", 424);

    println!("inside {}", SETTINGS.read()?.get::<i32>("property")?);

    Ok(())
}

fn run_app() -> Result<(), Box<Error>> {
    try_settings().unwrap();

    // let args: Vec<String> = env::args().collect();

    // let command = &args[1];

    // match command.as_str() {
    //     "setup" => setup(),
    //     //"init" => init(),
    //     _ => setup(),
    // }

    // setup::setup::setup();
    // init::init::confirm_language();

    // let document =
    //     fetch_document("https://open.kattis.com/problems/3dprinter".to_string()).unwrap();

    // let (texts, tests) = parse_content(document);

    // match init(texts, tests) {
    //     Ok(_) => (),
    //     Err(_) => (),
    // }

    Ok(())
}
