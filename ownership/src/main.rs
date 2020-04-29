use std::io;

// s is not valid as it hasn't been declared
fn s_scope(){
    let s = "This is a string"; // valid as it was declared
    // s is valid here as its in scope
}
// s is not valid here as the function is out of the stack

fn string_types(){
    // the s_literal is immutable
    let s_literal = "I am a string literal";
    // the s_type is mutable
    let s_type = String::from("I am not a string literal");
    s.push_str(", I am much cooler"); // we can mutate its value!
}


fn main() {
    println!("Hello, world!");
}
