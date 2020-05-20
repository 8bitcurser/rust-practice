use std::collections::HashMap;
// I had to name the module as the file
mod mean_mode;
mod pig_latin;
mod departments;
// after bringin the namespace use pub use to name the module inside the file
pub use crate::mean_mode::{mean, mode};
pub use crate::pig_latin::pig;
pub use crate::departments::abm;

fn main() {
    let arr = [1, 2, 3, 4, 5];
    // send reference of arr
    println!("The mean is {}", mean::mean(&arr));
    println!("The array used was: {:?}", arr);
    let arr_mode = [1, 1, 1, 2, 2, 2, 2, 3];
    // send reference of arr_mode
    println!("The mode is {}", mode::mode(&arr_mode));
    println!("The array used was {:?}", arr_mode);

    println!("{}", pig::pig(String::from("apple")));
    println!("{}", pig::pig(String::from("potatoe")));
    let mut company = HashMap::new();
    let mut dep_people: Vec<String> = vec![String::from("blah")];
    company.insert("Test", &mut dep_people);
    abm::add("Bleh", &mut company);
    // company.insert(String::from("test"), &dep_people);
    
}
