#![allow(non_snake_case)]

use std::env;
use std::process;

use test_rust;
use test_rust::Config;


 fn main() {
	
	let args: Vec<String> = env::args().collect();

	let Config  = Config::new(&args).unwrap_or_else(|err| {
		println!("Problem parsing exist: {}", err);
		process::exit(1);
	});

	if let Err(e) = test_rust::run(Config) {
		println!("{}", e );
		process::exit(1);
	};

}
