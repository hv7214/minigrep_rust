#![allow(non_snake_case)]

use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
 	  pub query: String,
 	  pub filename: String,
    pub case_sensitive: bool,
 }

impl Config {

   	pub fn new(args: &[String]) -> Result<Config, &'static str> {
   	 
   	    if args.len() <3 {

   	 	    return Err("not enough arguments!");

   	    }
   	    else {

   	    let query    = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("true").is_err();

        Ok(Config { query, filename, case_sensitive })
		      }
     }
}

pub fn run(Config: Config)-> Result< (), Box<dyn Error> > {
	 
    let contents = fs::read_to_string(Config.filename)?;

    let results = 
      if Config.case_sensitive {
          search(&Config.query, &contents)
      } 
      else {
          search_case_insensitive(&Config.query, &contents)
      };

     for line in results {
       println!("{}", line );
     }

     Ok(())
}

pub fn search<'a>(query: &str,contents: &'a str) -> Vec<&'a str> {
    
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str,contents: &'a str) -> Vec<&'a str> {
    
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}