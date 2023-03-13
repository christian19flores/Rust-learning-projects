mod args;

use std::process;
use args::Args;
use ROT_13::Config;

fn main() {
    let args = Args::new();
    println!("{:?}", args);

    let config = Config::build(args.option, args.text).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = config.run() {
        println!("Application error: {e}");
        process::exit(1);
    }
}