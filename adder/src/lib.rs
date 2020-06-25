#[cfg(test)]

pub fn add_two(a: i32) -> i32 {
    a + 2
}

mod tests {
    // Without this line we wouldn't be able to test add_two
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn fail_test() {
        assert!(!true, "Value was not true");
    }

    #[test]
    fn pass_test() {
        assert!(!false);
    }

    #[test]
    fn is_eq(){
        assert_eq!(1 ,1);
    }

    #[test]
    fn is_neq(){
        assert_ne!(1, 2);
    }


    #[test]
    fn is_four(){
        assert_eq!(add_two(2), 4);
    }
}
