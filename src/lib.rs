use std::env;
use std::error::Error;
use std::fs;

pub struct Config{
    pub query:String,
    pub file_path:String,
    pub ignore_case:bool,

}
impl Config{

    pub fn build(args:&[String])->Result<Config,&'static str>{
        if args.len() < 3 {
            return Err("not enough params");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path,ignore_case })
    }
}


pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case{
        case_insensitive(&config.query,&contents)
    }else{
        search(&config.query,&contents)
    };
    for line in search(&config.query,&contents){
        println!("{line}");
    }
    Ok(())
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
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }


    #[test]
    fn case_insensitive(){
        let query = "rUSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:","Trust me."],
            search_case_insensitive(query,contents)
        )
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            result.push(line)
        }
    }
    result
}

pub fn case_insensitive<'a>(query:&str,contents:&'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}