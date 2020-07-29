use std::error::Error;
use std::fs;
use std::env;

//dyn means dynamic, this allows us to return different type of errors.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // returns a Result<String>
    // instead of panicking the error is sent to the caller so it can be
    // handled, this would be main.
    let contents = fs::read_to_string(config.filename)?;
    // is idiomatic we want the side effect which is reading the file
    // not a return itself.
    let _results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    println!("{:#?}", _results);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    // using clone is not optimal yet it provides clarity and the trade off
    // for this scenario is worth to change.
    // skip first poitnless arg
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get any query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get any filename string"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }
}

// this means the resulting vector is "binded" to the lifetime of the contents
// string passed, this resulting vector will have slices of the contents string
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let results = contents.lines().filter(|x| x.contains(query)).collect();
    results
}

pub fn search_case_insensitive<'a> (
    query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
#[path="./unit_tests.rs"]
mod unit_tests;
