use std::io::Read;
use std::fs::File;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

pub fn run(config: Config) {
    println!("{}", config.filename);
    let mut f = File::open(config.filename).expect("Unable to find {config.filename}");

    let mut content = String::new();
    f.read_to_string(&mut content).expect("Error reading file.");

    println!("{:?}", search(&config.query, &content));
}

fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new(); 
    
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
} 

