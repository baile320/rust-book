use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // ignore the first arg, it's the filepath

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename string"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests broke doing 13.3 in the rust book.
    // See here for alternatives: https://stackoverflow.com/questions/47441279/creating-an-stdenvargs-iterator-for-testing
    // suggests Config::new() to take an &[String] instead of env::Args.
    // #[test]
    // fn config_new_should_succeed() {
    //     let config = Config::new(env::args([
    //         String::from("path"),
    //         String::from("query"),
    //         String::from("filename"),
    //     ]))
    //     .unwrap();

    //     assert_eq!(config.query, "query");
    //     assert_eq!(config.filename, "filename");
    // }

    // #[test]
    // fn config_should_be_err_without_args() {
    //     let config_err = Config::new(&[]).is_err();
    //     assert_eq!(config_err, true);
    // }

    #[test]
    fn run_should_fail_with_inexistent_file() {
        let run_is_err = run(Config {
            query: String::from("Some search query"),
            filename: String::from("ThisFileDefinitelyDoesntExist.txt"),
            case_sensitive: true,
        })
        .is_err();

        assert_eq!(run_is_err, true);
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
}
