use std::error::Error;
use std::fs;
use std::env;

// 重构5
// fn build(args: &[String]) -> Result<Config, &'static str>
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // 重构8
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }else {
        search(&config.query, &contents)
    };

    // 重构7
        // .expect("Should have been able to read the file");
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    // println!("With text:\n{contents}");

    // 重构5
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// 重构1
// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let file_path = &args[2];
//
//     (query, file_path)
// }

// 重构2
// fn parse_config(args: &[String]) -> Config {
//     let query = &args[1];
//     let file_path = &args[2];
//
//     Config {
//         query, file_path
//     }
// }

impl Config {
    // 修改new为build
    // fn new(args: &[String]) -> Config

    // 重构3
    // fn build(args: &[String]) -> Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        // feature：改进报错信息
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // Config {
        //     query, file_path
        // }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
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

        assert_eq!(
            vec!["safe, fast, productive."], search(query, contents)
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
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // vec![]
    let mut results = Vec::new();

    for line in contents.lines() {
        // do something with line
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}