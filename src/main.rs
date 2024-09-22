use clap::Parser;
use regex::Regex;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Json File to parse
    #[arg(value_parser = validate_json_file)]
    file_name: String,
}

fn get_content(file_name: &String) -> String {
    let a: String = match fs::read_to_string(&file_name) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        }
    };

    return a;
}

fn validate_json_file(file_name: &str) -> Result<String, String> {
    let re = Regex::new(r"\.json$").unwrap();

    if re.is_match(&file_name) {
        Ok(file_name.to_string())
    } else {
        Err(String::from("The file must have a .json extension"))
    }
}

fn main() {
    let Args { file_name } = Args::parse();

    let content = get_content(&file_name);

    println!("{content}");
}
