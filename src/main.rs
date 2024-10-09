use std::env;
use std::fs;
use std::io::{self, Write, stdin};
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use directories::ProjectDirs;
use serde_json::{Value, json, from_str, from_reader};
use rpassword::read_password;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "tony";
const APPLICATION: &str = "getcmd";

const CONFIG_FILENAME: &str = "conf.json";
const API_KEY_KEY: &str = "api_key";

const CMD_STR: &str = "`";
const PROMPT_PREFIX: &str = "Respond ONLY with the command to run to perform the following objective on Ubuntu Linux, surrounded by the '`' character:";
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
    let config_json: Value = get_config(); 

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

    // extract response
    let response_json: Value = from_str(&response).unwrap();
    let response_command: String = extract_command_from_response(&response_json);

    // ask for confirmation
    println!("getcmd returned the following command:\n\n{}\n\nrun it? (y/n)", response_command);
    io::stdout().flush().unwrap(); // flush output
    let mut continue_response = String::new();
    stdin().read_line(&mut continue_response).expect("did not enter a correct string");
    if continue_response.trim() != "y" { // exit if user does not want to run command
        println!("exiting...");
        exit(0);
    }
    
    // execute command and print output
    let mut child= Command::new(response_command).envs(env::vars()).stdout(Stdio::piped()).spawn().expect("failed to execute command");
    let output = child.wait_with_output().expect("failed to read stdout");
    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
}

fn get_config() -> Value {
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
    let mut config_json: Value = from_str("{}").unwrap();
    if config_dir.exists() {
        let config_file = fs::File::open(&config_dir).expect("config file");
        config_json = match from_reader(config_file) {
            Ok(conf) => conf,
            Err(_) => from_str("{}").unwrap(),
        };
    }

    if config_json.to_string() == "{}" {
        // create config 
        println!("OpenAI API key not configured. Please paste your key below:");
        io::stdout().flush().unwrap(); // flush output
        let api_key = read_password().expect("read input");
        if let Some(obj) = config_json.as_object_mut() {
            obj.insert(API_KEY_KEY.to_string(), Value::String(api_key));
        }

        // write config file
        fs::write(&config_dir, config_json.to_string()).expect("write JSON config");
    }

    config_json
}

fn extract_command_from_response (response: &Value) -> String {
    let choices: &Value = response.get("choices").expect("OpenAI API response choices array");
    let choice: &Value = choices.get(0).expect("OpenAI response choice object");
    let message: &Value = choice.get("message").expect("OpenAI response choice message");
    let content: String = message.get("content").expect("OpenAI response message content").to_string();

    content.replace(CMD_STR, "")
}
