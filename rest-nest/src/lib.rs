#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// this is a crate root, the content of this file forms a module named 
// crate at the root of the crates module structure
//module definition
mod front_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}



fn server_order() {}

mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();
        super::server_order();
    }
    fn cook_order() {}
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Seeds");
    meal.toast = String::from("Wheat");
    println!("I will have {} toasts", meal.toast);
    // this will explode as seasonal fruit is private lets comment it
    // meal.seasonal_fruit = String::from("banana");
}

// super is elevate one level of the scope server order is used
// by fix_incorrect_order but server_order is defined at the same scope
// of back_of_house, super elevates the path to the same level of
// back_of_house, this could also be done by doing
// crate::server_order, but cook order is defined at the same scope
// as incorrect_order and incorrect_order knows its scope as a child
// module therefor has access to cook order

pub fn eat_at_rest() {
    // if the sub module and the function within weren't public
    // we wouldn't be able to import them
    // absolute path
    crate::front_house::hosting::add_to_waitlist();
    // relative path
    front_house::hosting::add_to_waitlist();
}
