use std::{env, process};
use tracker::{run, Args};

fn main() {
    let args = Args::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
