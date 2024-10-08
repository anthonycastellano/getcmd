use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use directories::ProjectDirs;
use serde_json;
use serde_json::{Value, json};
use rpassword::read_password;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "tony";
const APPLICATION: &str = "getcmd";
const CONFIG_FILENAME: &str = "conf.json";
const API_KEY_KEY: &str = "api_key";
const PROMPT_PREFIX: &str = "Respond with ONLY the command to run to perform the following task on Ubuntu Linux, and nothing else: ";
const OPENAI_URL: &str = "https://api.openai.com";
const OPENAI_CHAT_PATH: &str = "/v1/chat/completions";
const OPENAI_CHAT_MODEL: &str = "gpt-4o-mini";

fn main() {
    // grab args
    let args: Vec<String> = env::args().collect();

    // validate input
    if args.len() <= 1 {
        println!("Error: An instruction must be provided.");
        exit(1);
    }

    // set up api key
    let config_json: serde_json::Value = get_config(); 

    // combine non-flag args into string
    let prompt: String = format!("{}{}", PROMPT_PREFIX, args[1..].join(" ").to_string());

    // set up request
    let url: String = format!("{}{}", OPENAI_URL, OPENAI_CHAT_PATH);
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", config_json.get(API_KEY_KEY).unwrap().as_str().unwrap())).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
    let body = json!({
        "model": OPENAI_CHAT_MODEL,
        "messages": [{
            "role": "user",
            "content": &prompt
        }]
    });
    
    // make request
    let client = Client::new();
    let response = match client.post(url).headers(headers).json(&body).send() {
        Ok(res) => res.text().unwrap(),
        Err(e) => {
            println!("Error while making request: {}", e);
            String::from("{}")
        },
    };

    let response_json: Value = serde_json::from_str(&response).unwrap();
    println!("{}", response_json);

}

fn get_config() -> serde_json::Value {
    // get config dir
    let mut config_dir: PathBuf = match ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        Some(proj_dirs) => proj_dirs.config_dir().to_path_buf(),
        None => Path::new("").to_path_buf(),
    };
    if config_dir.to_str().unwrap() == "" {
        println!("Error: Unable to get config dir.");
        exit(1);
    }

    // create config dir if it does not yet exist
    if !config_dir.exists() {
        match fs::create_dir_all(&config_dir) {
            Ok(_) => {},
            Err(e) => {
                println!("Error: Could not create config dir: {}", e);
                exit(1);
            },
        }

    }

    // read config file if it exists, create empty JSON object if it does not yet exist
    config_dir.push(CONFIG_FILENAME);
    let mut config_json: serde_json::Value = serde_json::from_str("{}").unwrap();
    if config_dir.exists() {
        let config_file = fs::File::open(&config_dir).expect("config file");
        config_json = match serde_json::from_reader(config_file) {
            Ok(conf) => conf,
            Err(_) => serde_json::from_str("{}").unwrap(),
        };
    }

    if config_json.to_string() == "{}" {
        // create config 
        println!("OpenAI API key not configured. Please paste your key below:");
        io::stdout().flush().unwrap();
        let api_key = read_password().expect("read input");
        if let Some(obj) = config_json.as_object_mut() {
            obj.insert(API_KEY_KEY.to_string(), serde_json::Value::String(api_key));
        }

        // write config file
        fs::write(&config_dir, config_json.to_string()).expect("write JSON config");
    }

    config_json
}
