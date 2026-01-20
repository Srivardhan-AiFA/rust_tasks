use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub search_string: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Please provide the required arguments");
        }
        let search_string = args[1].clone();
        let file_name = args[2].clone();

        let is_case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        Ok(Config {
            search_string,
            file_name,
            case_sensitive: is_case_sensitive,
        })
    }
}

pub fn get_file_contents(file_name: &String) -> Result<String, Box<dyn Error>> {
    let file_contents = fs::read_to_string(file_name)?;
    Ok(file_contents)
}

pub fn search_content_case_senstive<'a>(
    search_string: &str,
    file_content: &'a str,
) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in file_content.lines() {
        if line.contains(search_string) {
            result.push(line);
        }
    }
    result
}

pub fn search_content_case_insenstive<'a>(
    search_string: &str,
    file_content: &'a str,
) -> Vec<&'a str> {
    let search_string = search_string.to_lowercase();
    let mut result = Vec::new();
    for line in file_content.lines() {
        if line.to_lowercase().contains(&search_string) {
            result.push(line);
        }
    }
    result
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_senstive() {
        let search_string = "duct";
        let file_content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_content_case_senstive(search_string, file_content)
        )
    }

    #[test]
    fn case_insenstive() {
        let search_string = "rUsT";
        let file_content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_content_case_insenstive(search_string, file_content)
        )
    }
}
