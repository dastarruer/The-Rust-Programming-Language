use std::env;
use std::io::Read;
use std::process;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    run(config);
}

struct Config {
    filename: String,
    query: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        Ok(Config {
            filename: args[1].clone(),
            query: args[2].clone(),
        })
    }
}

fn run(config: Config) {
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
