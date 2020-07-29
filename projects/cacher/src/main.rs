use std::thread;
use std::collections::HashMap;
use std::time::Duration;


struct Cacher<T, A>
    where A: std::cmp::Eq + std::hash::Hash + Copy,
          T: Fn(A) -> A,
          
{
    calculation: T,
    values: HashMap<A, A>

}


impl<T, A> Cacher<T, A>
    where A: std::cmp::Eq + std::hash::Hash + Copy,
          T: Fn(A) -> A
{
    fn new(calculation: T) -> Cacher<T, A> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: A) -> A {
        let value = self.values.get(&arg);
        let v = match value {
            Some(v) => v,
            None => self.values.entry(arg).or_insert((self.calculation)(arg)),
        };
        *v
    }
}


fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next do {} situps",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }
}


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
 
}
