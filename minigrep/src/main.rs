extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;


fn main() {
    //  Though we very rarely need to annotate types in Rust, collect is one function you do often
    //  need to annotate because Rust isn’t able to infer what kind of collection you want.
    let args: Vec<String> = env::args().collect();
    // eprintln! sends string to stderr instead of stdout.
    eprintln!("{:?}", args);
    // unwrap_or_else runs the closure if the Result if of Err type.
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

