use std::env;
use std::process;

use minigrep::Config;

/// the main file must not hold the logic, thats the job of lib.rs
/// main should only call a run function that comes from the lib file
/// also, the command line logic can exist inside main as long as it doesn't
/// get too complicated. Having said this the resonsabilities are limited to
/// setting up any config, calling the run function, handling errors coming
/// from the run call and call the CLI parsing logic with the argument values.
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("This are the args passed => {:?}", args);
    // if we get an error we call the clousure
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing the arguments: {}", err);
        // exit code 1 = FAIL
        process::exit(1);
    });
    // instead of the unwrap we use the if let syntax, checking only if there 
    // is an err returned coming from run, also there is nothing to unwrap
    // we dont care of a return in case it all goes well remember the Ok(())
    // its a ok to the unit type.
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }

}


