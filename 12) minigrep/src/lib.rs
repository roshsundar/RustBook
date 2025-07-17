use std::{env, fs};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // The parsed args may not be what we expect, so we return a Result
    // where the Ok is a Config and the Err is an error string.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args, provide a word and filename.");
        }

        // The program's name is the first arg, args[0]
        let query = args[1].clone();
        let file_path = args[2].clone();

        // is_ok() evaluates the result to determine if the environmental var is set
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case
        })
    }   
}

/* The main logic of minigrep. Performs the search on the file and prints the lines containing the query.
*/
// In the Ok case, return the unit type (). In the Err case, a Box<dyn Error> means that
// the method can return any type that implements the Error trait.
//      • This is so that we can return various error values in different error cases.
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // Attempt to open the file and get the contents
    let contents = fs::read_to_string(config.file_path)?; // Recall the ? operator will, if Err, return the error val to the caller

    // Based on the config, run the appropriate search
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    for line in results{
        println!("{line}");
    }

    Ok(())
}

/* Perform a case-sensitive search on the text for the query.
A list of references to each line containing the query is returned. 
*/
// Specify that the lifetime of contents must live at least as long as the search results
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results  = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/* Perform a case-insensitive search on the text for the query.
A list of references to each line containing the query is returned. 
*/
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    let mut results  = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// The Test Driven Development (TDD) process can be an effective way of developing software.
// Write the tests first and then write the function. Then iterate to make sure the test passes.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape?";

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
            search_case_insensitive(query, contents),
        );
    }
}