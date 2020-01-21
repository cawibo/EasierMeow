pub mod setup {

    use directories::BaseDirs;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::io::{stdin, stdout, Write};
    use std::{env, fs};

    static FOLDER_NAME: &str = "Mjau"; // duplicate, PLEASE FIX. [init, setup]

    #[derive(Serialize, Deserialize, Debug)]
    struct Config {
        author: String,
        email: String,
        default_language: String,
    }
    pub fn setup() -> Result<(), std::io::Error> {
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
}
