# url_parser_on_rust

## Overview

This project implements a URL parser in Rust using the [`pest`](https://pest.rs/) parser generator. The parser is designed to handle various components of a URL, including the scheme, host, port, path, query parameters, and fragment.

## Installation
To use parser in your Rust project, add it as a dependency in your Cargo.toml file:
```toml
[dependencies]
url_parser_on_rust = "1.0.0"
```
Select the current version on crates.io: [`url_parser_on_rust`](https://crates.io/crates/url_parser_on_rust)
## Usage
### As lib
First, import the library in your Rust code:
```rust
use url_parser_on_rust::parse_url;
```
Then, you can convert Markdown text to HTML using the parse_markdown function:
```rust
fn main() {
    let url_string = "https://www.example.com/path?query=some";
    match parse_url(url_string) {
        Ok(parsed_url) => println!("Successfully parsed URL: {:?}", parsed_url),
        Err(err) => eprintln!("Error parsing URL: {}", err),
    }
}
```
### As CLI
You can also use this parser as a command-line tool. To do this, first install the Rust toolchain on your system. Then, clone the repository and build the project:
```bash
$ make help
$ make run path/to/file/file.txt 
or
$ make run http://urlfrom.com
```

## Features

- **Flexible URL Parsing:** The parser supports a wide range of URL formats, allowing for flexibility in the structure of the provided URLs.

- **Optional Components:** Components like path, query parameters, and fragments are optional, providing versatility in handling URLs with or without these elements.

- **Error Handling:** Proper error handling is implemented using the `Result` type, ensuring graceful handling of parsing errors.

## Project Structure

The project is structured as follows:

- **`src/lib.rs`:** Contains the implementation of the URL parser, including the grammar rules and parsing logic.

- **`src/main.rs`:** A simple executable that demonstrates the usage of the parser by parsing example URLs.

- **`tests/tests.rs`:** Unit tests for various scenarios, ensuring the correctness of the URL parsing logic.

- **`Cargo.toml`:** The project configuration file specifying dependencies and other metadata.

## How to Run

To run the project, execute the following command:

```bash
$ cargo run -- parse http://example.com
or
$ cargo run -- parse path/to/file/file.txt
```
This will execute the main.rs file, demonstrating the parsing of example URLs.

## Unit Tests

The project includes a comprehensive set of unit tests in the tests.rs file. To run the tests, use the following command:

```bash
cargo test
```
## Dependencies

The project relies on the pest crate for parsing and pest_derive for procedural macro support. Ensure these dependencies are correctly specified in the Cargo.toml file.


```toml
[dependencies]
pest = "2.7.5"
pest_derive = "2.7.5"  
clap = "2.33"
thiserror = "1.0.0" 
```

## URL Grammar
```pest
url = { scheme ~ "://" ~ host ~ (":" ~ port)? ~ (path ~ ("/" ~ file)? ~ ("?" ~ query)? ~ ("#" ~ fragment)?)? }
scheme = { identifier_with_optional_dot }
host = { identifier_with_optional_dot }
port = { number }
path = { "/" ~ identifier ~ ("/" ~ identifier)* }
query = { identifier ~ ("=" ~ identifier)* }
fragment = { identifier }
file = { identifier ~ ("." ~ identifier)* }

identifier = { ASCII_ALPHANUMERIC+ }
identifier_with_optional_dot = { identifier ~ ( "." ~ ASCII_ALPHANUMERIC+)* }
number = { DIGIT+ }

WHITESPACE = _{ " " | "\n" | "\t" }
ASCII_ALPHANUMERIC = _{ ASCII_ALPHABET | DIGIT }
ASCII_ALPHABET = _{ 'a'..'z' | 'A'..'Z' }
DIGIT = _{ '0'..'9' }
```
