use std::process;

use clap::Parser;

use sha2_utils::{sha256, sha512};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    message: String,

    #[arg(long)]
    hash_algorithm: String,
}

fn main() {
    let args = Args::parse();

    let message = args.message;

    let result = match args.hash_algorithm.to_lowercase().as_str() {
        "sha256" => Some(sha256(&message.into_bytes().to_vec())),
        "sha512" => Some(sha512(&message.into_bytes().to_vec())),
        _ => None,
    };

    if result.is_none() {
        println!("Algorithm {} is not valid!", args.hash_algorithm);
        process::exit(1);
    }

    println!("{}", result.unwrap());
}
