use std::env;
use std::fs::File;
use std::io::Read;

/// Stores the commandline arguments given by the user.
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, &'static str> {
        // If there are less than three arguments, the program cannot run
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        /*
        If there is no env variable called CASE_INSENSITIVE, .is_err will return true, meaning that we should search with case sensitivity.
        */
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        // TODO: Remove cloning here
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

/// Run main logic
pub fn run(config: &Config) {
    // Read file content
    let content = read_from_file(&config.filename);

    // Search for word in file and print results
    let results = match config.case_sensitive {
        true => search(&config.query, &content),
        false => search_case_insensitive(&config.query, &content),
    };
    print_results(results);
}

/// Return file content
fn read_from_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("Unable to find file, try again.");

    let mut content = String::new();
    f.read_to_string(&mut content).expect("Error reading file.");

    content
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
/// Search for a word in a given file's content, disregarding case
fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    // Go through each line
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            // Add the line to the results
            results.push(line);
        }
    }

    results
}

fn print_results(results: Vec<&str>) {
    for result in results.iter() {
        println!("{}", result);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "Rust: \nsafe, fast, productive.\nPick three.
        ";

        assert_eq!(vec!["safe, fast, productive."], search(query, content))
    }

    #[test]
    fn case_insensitive() {
        let query = "dUcT";
        let content = "Rust: \nsafe, fast, productive.\nPick three.
        ";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, content)
        )
    }
}
