use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;
use std::error::Error;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Error pasing arguments: {}", err);
        process::exit(1);
    });

    let file = File::new(&config).unwrap_or_else(|err|{
        println!("Error reading file: {}", err);
        process::exit(1);
    });
    println!("Read {} bytes of file: {}", file.length, file.filename);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

//    run(&file);
    file.show_contents();
    file.word_count();
}


fn run(file: &File){

}

struct File{
    filename: String,
    contents: Vec<u8>,
    length: usize,
}

impl File{
    fn new(config: &Config) -> Result<File, Box<dyn Error>>{
        let contents = fs::read(&config.filename)?;
        return Ok(File {filename: config.filename.clone(), contents: contents.to_owned(), length: contents.len()});
    }

    fn word_count(&self){
        let mut map = HashMap::new();      
        let re = Regex::new(r"/[^a-zA-Z ]/g").unwrap();
        let ocontents = String::from_utf8(self.contents.clone()).unwrap().to_lowercase();
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

    fn show_contents(&self){
        println!("With text:\n{}", String::from_utf8(self.contents.clone()).unwrap());      
    }
}

struct Config{
    query: String,
    filename: String,
}

impl Config{
    fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() < 3{
            return Err("Not enough arguments");
        }

        // if args.len() > 3{
        //    return Err("Too many arguments");
        // }
        return Ok(Config { query: String::from(&args[1]), filename: String::from(&args[2]) });
    }    
}

