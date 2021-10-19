use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), std::io::Error> {

	// '?' means that, if the Result is an Err -> return it
	let contents: String  = fs::read_to_string(config.filename)?;

	let results: Vec<&str> = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};

	for line in results {
		println!("{}", line);
	}
	return Ok(());
}

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	// need to set the lifetime of &str to 'static, as we are no longer passing a reference as a parameter
	pub fn new(mut args: env::Args) -> Result<Config, &'static str> {

		args.next();

		let query: String = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a query string")
		};

		let filename: String = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a file name")
		};

		let case_sensitive: bool = match env::var("CASE_SENSITIVE") {
			Err(_) => false,
			Ok(value) => value.parse().unwrap_or(false) 
		};
		return Ok(Config { query, filename, case_sensitive });
	}
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	return contents
		.lines()
		.filter(|line| line.contains(query))
		.collect();
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query: String = query.to_lowercase();
	return contents
		.lines()
		.filter(|line| line.to_lowercase().contains(&query))
		.collect();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let query: &str = "duct";
		let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}
	#[test]
	fn case_insensitive() {
		let query: &str = "rUsT";
		let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
	}
}