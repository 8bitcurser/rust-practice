use std::io::{self, Write};

fn input() -> u32 {
    let mut option = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut option).expect("Can't read line");
    let option: u32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Only numbers please");
            0
        }
    };
    option
}

fn input_float() -> f32{
    let mut option = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut option).expect("Can't read line");
    let option: f32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Only numbers please");
            0.0
        }
    };
    option
}


fn menu(){
    println!("==== Temperature converter ====");
    println!("Input 1 for Celsius to Farenheit");
    println!("Input 2 for Farenheit to Celsius");
    println!("Input 3 to exit");
}

fn main() {
    menu(); 
    loop {
        let option = input(); 
        if option == 1 {
            println!("=== Celsius to Farenheit === ");
            let val = input_float();
            let val = (val * (9.0 / 5.0)) + 32.0;
            println!("{} F", val);
            menu();
        } else if option == 2 {
            println!("Farenheit to Celsius!");
            let _val = input_float();
            let _val = (_val - 32.0) * (5.0 / 9.0);
            println!("{} C", _val);
            menu();
        } else if option == 3 {
            println!("Exit!");
            break;
        } else {
            continue;
        }
    }
}
