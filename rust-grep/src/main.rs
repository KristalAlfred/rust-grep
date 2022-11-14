use std::env;

use rust_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|_| {
        std::process::exit(1);
    });

    if let Err(_) = rust_grep::run(config) {
        std::process::exit(1);
    }
}