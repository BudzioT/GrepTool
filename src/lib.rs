use std::error::Error;
use std::{env, fs};


pub struct Parser {
    pub query: String,
    pub source: String,
    pub ignore_case: bool
}

impl Parser {
    // Create a new Parser
    pub fn build(args: &[String]) -> Result<Parser, &str> {
        // Make sure that enough arguments are provided
        if args.len() < 3 {
            return Err("Error: Not enough arguments provided");
        }

        // Return Parser with the correct fields set
        Ok(Parser {
            query: args[1].clone(),
            source: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok()
            })
    }
}

// Run the Grep Tool
pub fn run(parser: Parser) -> Result<(), Box<dyn Error>> {
    // Read the file
    let source: String = fs::read_to_string(parser.source)?;
    let query: String = parser.query;

    // Search the text and print all found lines, adjust the case-sensitivity
    let result = if parser.ignore_case {
        insensitive_search(&query, &source)
    }
    else {
        search(&query, &source)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

// Search for the given query and return all the lines it appears in
pub fn search<'a>(query: &str, source: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    // Go through each line, check if the query appears and append it to the result
    for line in source.lines() {
        if line.contains(query) {
          result.push(line);
        }
    }

    result
}

// Search for lines with given query, ignoring case of letters
pub fn insensitive_search<'a>(query: &str, source: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    // Shadow the query string to a lowercase one
    let query = query.to_lowercase();

    // Search and push lines containing the query to the result vector, ignoring the case
    for line in source.lines() {
        if line.to_lowercase().contains(query.as_str()) {
            result.push(line);
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    // Try to get the correct text after finding
    #[test]
    fn find_test() {
        let query: &str = "Then";
        let text: &str = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

        let result = vec!["Then there's a pair of us - don't tell!"];

        assert_eq!(result, search(query, text));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "RuSt";
        let text: &str = "Rust is awesome\nTesting testing\nTesting rust\nEnd of file";
        let result = vec!["Rust is awesome", "Testing rust"];

        assert_eq!(result, insensitive_search(query, text));
    }
}