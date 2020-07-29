
pub mod mean{
    pub fn mean(ints: &[u32]) -> u32 {
        let mut vec_ints: Vec<u32> = vec![];
        for int in ints.iter() {
            vec_ints.push(*int);
        }
        let mut sum = 0;
        for int in &vec_ints {
            sum += int;
        }
        // cast to int
        sum / vec_ints.len() as u32
    }
}

pub mod mode{
    // imports go inside the modules, if imported outside mod declaration
    // an error rises
    use std::collections::HashMap;
    pub fn mode(ints: &[u32]) -> u32 {
        let mut mode = HashMap::new();
        for int in ints.iter() {
            let count = mode.entry(*int).or_insert(1);
            *count += 1;

        }
        let mut max = 0;
        let mut key = 0;
        for (k, v) in &mode {
            // had to deference to compare value against value
            // and later to overwrite the variable
            if *v > max {
                max = *v;
                key = *k;
            }
        }
        key

    }
}
