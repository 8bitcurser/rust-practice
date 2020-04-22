use std::io;
// Rng is a trait that defines methods that random number gens implements
// if not in scope we wouldnt be able to use them.
use rand::Rng;
// provides the Ordering type which is another enum as Result
use std::cmp::Ordering;

fn main() {
    println!("Please input your guess!");
    // thread_rng provides a random number generator that is
    // local to the current thread and acquires a seed provided by the
    // OS.
    // gen_range is called on the thread_rng  gen and its defined by the
    // Rng trait
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
    // Function that returns a new inst of String 
    // ::new() means new is an associated function from String
    // associated functions are implemented at a Type such as String.
    // This is equivalent to a static method.
        let mut guess = String::new();
    // call stdin function the io module at the std library
    // stdin returns an instance of the Stdin Type
    // read_line is an associated function from this type
    // we need to pass the reference of a string to it so it can store
    // the value inputed by the user.
    //
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    // read_line places what was typed inside the string but also returns a
    // Result Type from the io module, expect is an associated method from
    // it, in case Result provides an Err the program will fail and return 
    // the message provided.
    // 
    // shadow the guess var and turn it into an unsigned int
    // it grabs the old guess varaible sanitize it and reassing it to
    // an inmutable version of it with the same name
    // trim takes the whitespaces surrounding the string 
    // parse turns the string into the type of the variable we defined
    // parse will produce a Result type if successful the Ok will be returned
    // if not the Err variant will be provided
        let guess: u32 = match guess.trim().parse() {
            // this is a better way to handle the variants of the Result
            // provided by parse. 
            // If its ok return the number if not, send a friendly message
            // and ignore the value. Expect would have made the program 
            // crash which is not user friendly at all.
            Ok(num) => num,
            Err(_) => {
                // the _ is a catchall value
                println!("Please input a number");
                continue;
            },
        };
    // If all ok Result will have an Ok value and return that
        println!("You guessed {}", guess);
    
    // cmp will produce an Ordering match will check the Ordering obtained
    // at each one of its arms and run the code that fits the pattern.

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
