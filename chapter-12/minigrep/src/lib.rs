use std::env;
use std::fs::File;
use std::io::Read;

/// Stores the commandline arguments given by the user.
pub struct Config {
    queries: Vec<String>,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, &'static str> {
        let num_args = args.len();

        // If there are less than three arguments, the program cannot run
        if num_args < 3 {
            return Err("Not enough arguments.");
        }

        let queries = args[1..num_args - 1]
            .iter()
            .map(|s| s.to_string())
            .collect();
        // TODO: Remove cloning here
        let filename = args[num_args - 1].clone();

        /*
        If there is no env variable called CASE_INSENSITIVE, .is_err will return true, meaning that we should search with case sensitivity.
        */
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            queries,
            filename,
            case_sensitive,
        })
    }
}

/// Run main logic
pub fn run(config: &Config) {
    // Read file content
    let content = read_from_file(&config.filename);
    let query_refs = config.queries.iter().map(|s| s.as_str()).collect();

    // Search for word in file and print results
    let results = match config.case_sensitive {
        true => search(query_refs, &content),
        false => search_case_insensitive(query_refs, &content),
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
fn search<'a>(queries: Vec<&'a str>, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // Go through each line
    for line in content.lines() {
        for query in queries.iter() {
            if line.contains(query) {
                // Add the line to the results
                results.push(line);
            }
        }
    }

    results
}
/// Search for a word in a given file's content, disregarding case
fn search_case_insensitive<'a>(queries: Vec<&'a str>, content: &'a str) -> Vec<&'a str> {
    let queries: Vec<_> = queries.into_iter().map(|s| s.to_lowercase()).collect();
    let mut results = Vec::new();

    // Go through each line
    for line in content.lines() {
        for query in queries.iter() {
            if line.to_lowercase().contains(query) {
                // Add the line to the results
                results.push(line);
            }
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
    fn one_query() {
        let query = vec!["duct"];
        let content = "Rust: \nsafe, fast, productive.\nPick three.
        ";

        assert_eq!(vec!["safe, fast, productive."], search(query, content))
    }

    #[test]
    fn one_query_case_insensitive() {
        let query = vec!["dUcT"];
        let content = "Rust: \nsafe, fast, productive.\nPick three.
        ";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, content)
        )
    }

    #[test]
    fn multiple_queries_case_insensitive() {
        let query = vec!["rUst", "dUcT"];
        let content = "Rust: \nsafe, fast, productive.\nPick three.
        ";

        assert_eq!(
            vec!["Rust: ", "safe, fast, productive."],
            search_case_insensitive(query, content)
        )
    }

    #[test]
    fn multiple_queries() {
        let query = vec!["Rust", "duct"];
        let content = "Rust: \nsafe, fast, productive.\nPick three.
        ";

        assert_eq!(
            vec!["Rust: ", "safe, fast, productive."],
            search(query, content)
        )
    }
}
