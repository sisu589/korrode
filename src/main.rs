// src/main.rs
mod engine;
mod hashers;
mod sources;

use hashers::Algo;
use std::{arch::x86_64::_MM_FROUND_TO_NEAREST_INT, env};

fn print_usage() {
    eprintln!(
        "Usage: korrode <algorithm> <target_hash> <mode> [options]\n\
        Algorithms: sha256 | md5 | bcrypt\n\
        Modes:\n\
        dict <path-to-wordlist>\n\
        bruteforce <digits>\n\
        Example:\n\
        korrode sha256 d2d2 dict ./korrodyou.txt"
    );
}

fn main() {
    // Very lightweight CLI parsing
    // Possible upgrade to 'clap'
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        print_usage();
        return;
    }

    // *** --- Parse Algorithm --- ***
    let algo = match args[1].as_str() {
        "sha256" => Algo::Sha256,
        "md5" => Algo::Md5,
        "bcrypt" => Algo::Bcrypt,
        _ => {
            eprintln!("Unsupported algorithm");
            return;
        }
    };

    // *** --- Target Hash --- ***
    let target_hash = &args[2];

    // *** --- Mode Selection --- ***
    let mode = &args[3];
    let result = match (mode.as_str(), &args[4..]) {
        ("dict", [path]) => {
            let iter = sources::dict_iter(path);
            engine::crack(algo, target_hash, iter)
        }
        ("bruteforce", [digits_str]) => {
            let digits: usize = digits_str
                .parse()
                .expect("Digits argument must be a positvie integer");
            let iter = sources::brute_force_numeric(digits);
            engine::crack(algo, target_hash, iter)
        }
        _ => {
            eprint!("Invalid mode or missing arguments");
            return;
        }
    };

    // *** --- Output --- ***
    match result {
        Some(pw) => println!("PASSWORD FOUND: {}", pw),
        None => println!("NO MATCH FOUND. Update candidate space.")
    }
}
