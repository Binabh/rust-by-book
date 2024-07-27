use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });
    if  let Err(e) = run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
fn run(config:  Config)->Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    let lines =  search(&config.query, &contents);
    for line in lines{
        println!("{}",line);
    }
    Ok(())
}
struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }
        let query = args[2].clone();
        let file_path = args[1].clone();
        Ok(Config { query, file_path })
    }
}
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|sentence| sentence.contains(query))
        .collect()
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
