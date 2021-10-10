use std::env;
use std::process;

use minigrep::Config;

fn main() {
	
	// env::args() returns an iterator and collect() turn the iterator into a collection
	let args: Vec<String> = env::args().collect();
	
	let config: Config = Config::new(&args).unwrap_or_else(|err| {
		// eprintln! printss to stderr instead of stdout
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);
	});

	if let Err(e) = minigrep::run(config) {
		eprintln!("Application error: {}", e);
		process::exit(1);
	}
}
