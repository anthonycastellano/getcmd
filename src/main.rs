use std::env;
use std::path::{Path, PathBuf};
use std::process::exit;
use directories::ProjectDirs;


fn main() {
    // grab args
    let args: Vec<String> = env::args().collect();

    // validate input
    if args.len() <= 1 {
        println!("Error: An instruction must be provided");
        exit(1);
    }

    // check config for api key
    let config_dir: PathBuf = match ProjectDirs::from("com","tony", "getcmd") {
        Some(proj_dirs) => proj_dirs.config_dir().to_path_buf(),
        None => Path::new("").to_path_buf(),
    };
    if config_dir.to_str().unwrap() == "" {
        println!("Error: Unable to get config dir");
        exit(1);
    }

    // combine non-flag args into string
    let prompt: String = args[1..].join(" ").to_string();

    println!("{:?}", prompt)
}
