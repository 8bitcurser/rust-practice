
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    // Without this line we wouldn't be able to test add_two
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
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

    // as it results a variant it can not have
    // the should panic attribute
    #[test]
    fn it_works_2() -> Result<(), String>{
        if 2+2 == 4 {
            // if test passes
            Ok(())
        }else {
            // if test fails
            Err(String::from("two plus two does not add to four"))
        }
    }

    #[test]
    fn it_works_2_fail() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("Hey i failed"))
        }
    }
}
