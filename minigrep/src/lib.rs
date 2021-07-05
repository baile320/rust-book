use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text: {}", contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_should_succeed() {
        let config = Config::new(&[
            String::from("path"),
            String::from("query"),
            String::from("filename"),
        ])
        .unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    fn config_should_be_err_without_args() {
        let config_err = Config::new(&[]).is_err();
        assert_eq!(config_err, true);
    }

    #[test]
    fn run_should_fail_with_inexistent_file() {
        let run_is_err = run(Config {
            query: String::from("Some search query"),
            filename: String::from("ThisFileDefinitelyDoesntExist.txt"),
        })
        .is_err();

        assert_eq!(run_is_err, true);
    }
}
