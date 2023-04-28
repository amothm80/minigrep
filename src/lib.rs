use std::collections::HashMap;
use std::fs;
use std::error::Error;
use regex::Regex;
use std::env;

pub struct File{
    pub filename: String,
    pub contents: String,
    pub length: usize,
}

impl File{
    pub fn new(config: &Config) -> Result<File, Box<dyn Error>>{
        let contents = fs::read_to_string(&config.filename)?;
        return Ok(File {filename: config.filename.clone(), contents: contents.to_owned(), length: contents.len()});
    }

    pub fn word_count(&self){
        let mut map = HashMap::new();      
        let re = Regex::new(r"/[^a-zA-Z ]/g").unwrap();
        let ocontents = self.contents.to_lowercase();
        let contents = re.replace_all(ocontents.as_str(), "");
        //let contents = String::from_utf8(self.contents.clone()).unwrap().to_lowercase();
        for word in contents.split_whitespace(){
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        for (word,count) in map{
            println!("The word {} is mentioned {} time/s", word, count);
        }
    }

    pub fn show_contents(&self){
        println!("With text:\n{}", self.contents.clone());      
    }

    pub fn search(&self, query: &str)->Vec<&str>{
        let mut results = Vec::new();
        for line in self.contents.lines(){
            if line.contains(&query.to_lowercase()){
                results.push(line);
            }
        }
        return results;
    }    


// my case insensitive
    // pub fn search_case_insensitive(&self, query: &str)->Vec<String>{
    //     let mut results = Vec::new();
    //     for (index, line) in self.contents.to_lowercase().lines().enumerate(){
    //         if line.contains(&query.to_lowercase()){
    //             results.push(String::from(self.contents.lines().collect::<Vec<&str>>()[index]));
    //         }
    //     }
    //     return results;
    // }

//his case insensitive
    pub fn search_case_insensitive(&self, query: &str)->Vec<&str>{
        let mut results = Vec::new();
        let query = query.to_lowercase();
        for line in self.contents.lines(){
            if line.to_lowercase().contains(&query){
                results.push(line);
            }
        }
        return results;
    }    
}

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() < 3{
            return Err("Not enough arguments");
        }

        // if args.len() > 3{
        //    return Err("Too many arguments");
        // }
        return Ok(Config { query: String::from(&args[1]), 
                        filename: String::from(&args[2]),
                        case_sensitive: env::var("CASE_INSENSITIVE").is_err() });
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive(){
        let file = File {filename: String::from("test.txt"),
                         contents: String::from("
Rust:
safe, fast, productive.
Pick three.
Duct tape
"),
                        length: 50};
        let query = "duct";
        assert_eq!(vec!["safe, fast, productive."], file.search(query));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let file = File {filename: String::from("test.txt"),
                         contents: String::from("
Rust:
safe, fast, productive.
Pick three.
Trust me.
"),        
                         length: 50};
        
        assert_eq!(
            vec!["Rust:", "Trust me."],
            file.search_case_insensitive(query)
        );
    }
}