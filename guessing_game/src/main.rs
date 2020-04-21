use std::io;

fn main() {
    println!("Please input your guess!");
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
    // If all ok Result will have an Ok value and return that
    println!("You guessed {}", guess);
}
