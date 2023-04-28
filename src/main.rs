use std::env;
use std::process;

use minigrep::Config;
use minigrep::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Error pasing arguments: {}", err);
        process::exit(1);
    });

    let file = File::new(&config).unwrap_or_else(|err|{
        eprintln!("Error reading file: {}", err);
        process::exit(1);
    });
    println!("Read {} bytes of file: {}", file.length, file.filename);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if config.case_sensitive {
        println!("Results:\n{:?}",file.search(config.query.as_str()));
    }else{
        println!("Results:\n{:?}",file.search_case_insensitive(config.query.as_str()));
    }
    //file.word_count();
}



