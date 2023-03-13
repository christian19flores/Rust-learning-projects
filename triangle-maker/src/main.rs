use std::env;
use std::process;

use triangle_maker::Config;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = config.run() {
        println!("Application error: {e}");
        process::exit(1);
    }
}