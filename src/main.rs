use text_colorizer::*;
use std::env;
use std::fs;
use regex::Regex;

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let re = Regex::new(target)?;
    Ok(re.replace_all(text, replacement).to_string())
}

fn print_usage() {
    eprintln!("{} - change occurerences of one string into another",
                "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{}: Expected 4 arguments, got {}", "Error".red(), args.len());
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
    
    let data = match fs::read_to_string(&args.filename) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}: Failed to read file '{}': {}", "Error".red(), args.filename, e);
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(replaced_data) => replaced_data,
        Err(e) => {
            eprintln!("{}: Failed to replace text: {}", "Error".red(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: Failed to write to '{}': {}", "Error".red(), args.output, e);
            std::process::exit(1);
        }
    };
}