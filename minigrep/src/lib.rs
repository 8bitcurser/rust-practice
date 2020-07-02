use std::error::Error;
use std::fs;

//dyn means dynamic, this allows us to return different type of errors.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // returns a Result<String>
    // instead of panicking the error is sent to the caller so it can be
    // handled, this would be main.
    let contents = fs::read_to_string(config.filename)?;

    println!("\nThe file has the following content: \n{}", contents);
    // is idiomatic we want the side effect which is reading the file
    // not a return itself.
    Ok(())
}

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
    // using clone is not optimal yet it provides clarity and the trade off
    // for this scenario is worth to change.
        if args.len() < 3 {
            return Err("not enough arguments");
        } else if args.len() > 3 {
            return Err("too many arguments");
        } else {
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok(Config {query, filename})
        }
    }
}


#[cfg(test)]
#[path="./unit_tests.rs"]
mod unit_tests;
