use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::{self, Read};
use std::process;
use thiserror::Error;
use url_parser_on_rust::{parse_url, ParseUrlError};

#[derive(Debug, Error)]
enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("URL parsing error: {0}")]
    UrlParsingError(#[from] ParseUrlError),
    #[error("Invalid subcommand. Use '--help' for more information.")]
    InvalidSubcommand,
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::UrlParsingError(ParseUrlError::ParsingError(s.to_string()))
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let matches = App::new("URL Parser")
        .version("0.1.4")
        .author("Nazar Pashchuk(nazar.pashchuk@ukma.edu.ua)")
        .about("Parse URLs from a file or command-line input")
        .subcommand(
            SubCommand::with_name("parse")
                .about("Parse a URL")
                .arg(Arg::with_name("input").help("The URL or File path to parse")),
        )
        .subcommand(SubCommand::with_name("help").about("Print help information"))
        .subcommand(SubCommand::with_name("credits").about("Pring credit information"))
        .get_matches();

    match matches.subcommand() {
        ("parse", Some(parse_matches)) => {
            if let Some(input) = parse_matches.value_of("input") {
                if let Ok(url) = File::open(input) {
                    let mut content = String::new();
                    url.take(1024).read_to_string(&mut content)?;
                    let parsed_url = parse_url(&content)?;
                    println!("{:#?}", parsed_url);
                } else {
                    let parsed_url = parse_url(input)?;
                    println!("{:#?}", parsed_url);
                }
            } else {
                return Err(AppError::InvalidSubcommand);
            }
        }
        ("credits", _) => {
            println!("Credits: Nazar Pashchuk(mail: nazar.pashchuk@ukma.edu.ua; github: https://github.com/jester69m)");
        }
        ("help", _) => {
            println!("URL Parser CLI - Help Information\n");
            println!("1. Parse a URL from a String:\n");
            println!("```bash");
            println!("cargo run -- parse http://example.com");
            println!("```");
            println!("Replace `http://example.com` with the URL you want to parse.\n");
            println!("2. Parse a URL from a File:\n");
            println!("```bash");
            println!("cargo run -- parse path/to/your/file.txt");
            println!("```");
            println!("Ensure that the file at the specified path contains a valid URL.\n");
            println!("3. Help Information:\n");
            println!("```bash");
            println!("cargo run -- help");
            println!("```");
            println!("Prints information about how to use the URL Parser CLI.\n");
            println!("4. Credits Information:\n");
            println!("```bash");
            println!("cargo run -- credits");
            println!("```");
            println!("Prints credits information for the URL Parser CLI.");
        }
        _ => {
            return Err(AppError::InvalidSubcommand);
        }
    }

    Ok(())
}
