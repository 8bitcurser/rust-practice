use std::io;

fn first_word(s: &String) -> usize {
    // turn string into bytes
    let bytes = s.as_bytes();
    // turn bytes into an iterable and give it enumeration
    // meaning ((idx, ref to byte), (idx2, ref to byte))
    // we use &item cause we get a ref to the item by enumerate
    for (i, &item) in bytes.iter().enumerate() {
        // we look for the byte that represents a white space
        // notice the b as prefix as well as the single quotes
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// usize depends on the string, if the stirng is gone, the ref
// we will get will mean nothing
// we can clean a string by doing string.clear();


fn main() {
    println!("Hello, world!");
    let s = String::from("Hello World");
    let hello = &s[0..5];
    println!("{}", hello);
}
