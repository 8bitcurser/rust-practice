use std::io::{self, Write};


fn main() {
    let mut quantity = String::new();
    println!(
        "Input the amount of numbers you desire from the fibonacci sequence"
    );
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut quantity).expect(
        "I couldn't read that line \u{2639}");
    let quantity: u32 = match quantity.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Ooops that was not a number! \u{2639}");
            0
        }
    };
    let mut low_bound: u32 = 0;
    let mut high_bound: u32 = 0;
    for iter in 1..quantity+1 {
        let sum: u32 = low_bound + high_bound;
        println!("Iter {}: {}", iter, sum);
        if high_bound == 0 {
            low_bound = 1;
        } else {
            low_bound = high_bound;
        }
        high_bound = sum;
    }


}
