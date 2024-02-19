use std::error::Error;
use std::fs;
use std::env;
use std::env::Args;

pub struct Config {
    query: String,
    path: String,
    case_insensitive: bool
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next(); // First arg is the name of the program, so we are skipping it.

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string :("),
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a path :((")
        };

        let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();

        Ok( Config {
            query,
            path,
            case_insensitive
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| {line.contains(query)})
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.to_lowercase().lines()
        .filter(|line|{line.contains(query)})
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.path)?;

    let result = if config.case_insensitive {
        search_case_insensitive(&config.query, &content)
    }
    else {
        search(&config.query, &content)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
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

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
