use std::io::Read;
use std::fs::File;

/**
Stores the commandline arguments given by the user.
*/
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, &'static str> {
        // If there are less than three arguments, the program cannot run
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        // TODO: Remove cloning here
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

/// Run main logic
pub fn run(config: Config) {
    println!("{}", config.filename);
    let mut f = File::open(config.filename).expect("Unable to find {config.filename}");

    let mut content = String::new();
    f.read_to_string(&mut content).expect("Error reading file.");

    println!("{:?}", search(&config.query, &content));
}

/// Search for a word in a given file's content
fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new(); 
    
    // Go through each line
    for line in content.lines() {
        if line.contains(query) {
            // Add the line to the results
            results.push(line);
        }
    }

    results
} 

