// I had to name the module as the file
mod mean_mode;

// after bringin the namespace use pub use to name the module inside the file
pub use crate::mean_mode::{mean, mode};

fn main() {
    let arr = [1, 2, 3, 4, 5];
    // send reference of arr
    println!("The mean is {}", mean::mean(&arr));
    println!("The array used was: {:?}", arr);
    let arr_mode = [1, 1, 1, 2, 2, 2, 2, 3];
    // send reference of arr_mode
    println!("The mode is {}", mode::mode(&arr_mode));
    println!("The array used was {:?}", arr_mode);
}
