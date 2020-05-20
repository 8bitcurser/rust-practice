pub mod abm{
    use std::collections::HashMap;
    pub fn add(_input: &str, company: &mut HashMap<&str, &mut Vec<String>>) {
        println!("{}", _input);
        let mut empty_vec = Vec::new();
        company.entry(_input).or_insert(Vec::new());
        println!("{:#?}", company);
    }
    pub fn get(_input: String) {}
}
