use std::env;
use std::process::exit;

fn main() {
    // grab args
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Error: An instruction must be provided");
        exit(1);
    }

    // combine non-flag args into string
    let prompt: String = args[1..].join(" ").to_string();

    println!("{:?}", prompt)
}
