use std::env;
use std::process;

use GrepTool::{run, Parser};


fn main() {
    // Get the arguments
    let argv: Vec<String> = env::args().collect();
    // Parse arguments
    let parser = Parser::build(&argv).unwrap_or_else(|err| {
        eprintln!("Unable to Parse arguments, Error: {}", err);
        process::exit(-1);
    });

    // Run the Grep Tool
    if let Err(e) = run(parser) {
        eprintln!("Application error, Error: {}", e);
        process::exit(-1);
    }
}
