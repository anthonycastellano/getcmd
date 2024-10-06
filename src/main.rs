use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::exit;
use directories::ProjectDirs;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "tony";
const APPLICATION: &str = "getcmd";

fn main() {
    // grab args
    let args: Vec<String> = env::args().collect();

    // validate input
    if args.len() <= 1 {
        println!("Error: An instruction must be provided.");
        exit(1);
    }

    // get config
    let config_dir: PathBuf = match ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        Some(proj_dirs) => proj_dirs.config_dir().to_path_buf(),
        None => Path::new("").to_path_buf(),
    };
    if config_dir.to_str().unwrap() == "" {
        println!("Error: Unable to get config dir.");
        exit(1);
    }

    // create config dir if it does not yet exit
    if !config_dir.exists() {
        match fs::create_dir_all(config_dir) {
            Ok(_) => {},
            Err(e) => {
                println!("Error: Could not create config dir: {}", e);
                exit(1);
            },
        }

    }
    // let mut input_buf = String::new();
    // io::stdin().read_line(&mut input_buf)?;
    // println!("OpenAI API key not configured. Please paste your key below:")

    // combine non-flag args into string
    let prompt: String = args[1..].join(" ").to_string();

    println!("{:?}", prompt)
}
