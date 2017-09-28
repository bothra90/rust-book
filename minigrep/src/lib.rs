use std::fs::File;
use std::io::prelude::*; // Needed when dealing with I/O.
use std::error::Error;
use std::env; // for reading environment variables.

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // arg0 is the binary path. Ignore it.
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        // env::var is used to read environment variables. Here, we don't really care about the
        // value of the environment variable as long as it's set.
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    eprintln!("Searching for {}", config.query);
    eprintln!("In file {}", config.filename);
    // Read file filename.
    // '?' at the end of an operation that can fail immediately returns Error from the current
    // context.
    // Question: How does it automatically get put into a "Box".
    let mut f = File::open(config.filename)?;  // .expect("Failed to open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;  // .expect("something went wrong reading the file");
    // This is similar to conditional assignments in python.
    let results = if config.case_sensitive {
        search(&(config.query), &contents)
    } else {
        search_case_insensitive(&(config.query), &contents)
    };
    eprintln!("With text:\n{}", contents);
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

// Need to specify lifetime because we are passing 2 arguments. Output should have the same
// lifetime as that of the content.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = &(query.to_lowercase());
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

#[cfg(test)]
mod test;
