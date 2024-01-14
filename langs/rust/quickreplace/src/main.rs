use text_colorizer::*;
use std::env;
use std::fs;
use regex::Regex;

#[derive(Debug)] // generate extra code code that allows to format the args struct with {:?} in println!
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?; //compile regex, short circuit if fails
    Ok(regex.replace_all(text, replacement).to_string())
    //if replace, return new string, if not return a pointer to orignal text, to_string forces
}


fn print_usage() {
    eprintln!("{} - change occurences of one string into another",
	      "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}


fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect(); //Vec of args

    if args.len() != 4{
	print_usage();
	eprintln!("{} Wrong number of args, expect 4, got {}",
		  "Error:".red().bold(), args.len());
	std::process::exit(1);
    }

    Arguments {
	target: args[0].clone(),
	replacement: args[1].clone(),
	filename: args[2].clone(),
	output: args[3].clone(),
    }
}

fn main() {
    let args = parse_args();
    let data = match fs::read_to_string(&args.filename) { //match to process errors
	Ok(v) => v,
	Err(e) => {
	    eprintln!("{} failed to read from file '{}' : {:?}",
		      "Error:".red().bold(), args.filename, e);
	    std::process::exit(1);
	}
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data){
	Ok(v) => v,
	Err(e) => {
	    eprintln!("{} failed to replace text {:?}",
		      "Error:".red().bold(), e);
	    std::process::exit(1);
	}
    };
	
    match fs::write(&args.output, &replaced_data) {
	Ok(_) => {},
	Err(e) => {
	    eprintln!("{} Failed to write to file '{}' : {:?}",
		      "Error:".red().bold(), args.output, e);
	    std::process::exit(1);
	}
    };
}


