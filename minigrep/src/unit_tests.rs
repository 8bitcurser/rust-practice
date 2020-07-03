use super::*;

#[test]
#[should_panic(expected="too many arguments")]
fn test_too_many_args() {
    let args = vec![
        "1".to_string(), "2".to_string(),
        "3".to_string(), "4".to_string()
    ];
    if let Err(err) = Config::new(&args) {
        panic!(err);
    }
}

#[test]
#[should_panic(expected="not enough arguments")]
fn test_not_enough_args() {
    let args = vec![];
    if let Err(err) = Config::new(&args) {
        panic!(err);
    }
}

#[test]
fn test_proper_params() {
    let args = vec![
        "exec".to_string(),
        "query".to_string(),
        "path".to_string(),
    ];
    let config = Config::new(&args).unwrap_or_else(
        |err| {panic!()}
    );
    assert_eq!(config.query, "query".to_string());
    assert_eq!(config.filename, "path".to_string());
}


#[test]
fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents)
    );
}


#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents)
    );
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}
    
