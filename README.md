# URL Parser Project

## Overview

This project implements a URL parser in Rust using the [`pest`](https://pest.rs/) parser generator. The parser is designed to handle various components of a URL, including the scheme, host, port, path, query parameters, and fragment.

## Features

- **Flexible URL Parsing:** The parser supports a wide range of URL formats, allowing for flexibility in the structure of the provided URLs.

- **Optional Components:** Components like path, query parameters, and fragments are optional, providing versatility in handling URLs with or without these elements.

- **Error Handling:** Proper error handling is implemented using the `Result` type, ensuring graceful handling of parsing errors.

- **Command-Line Interface (CLI):** The project includes a simple command-line interface allowing users to input URLs for parsing.

## Project Structure

The project is structured as follows:

- **`src/lib.rs`:** Contains the implementation of the URL parser, including the grammar rules and parsing logic.

- **`src/main.rs`:** A simple executable that demonstrates the usage of the parser by parsing example URLs.

- **`tests/tests.rs`:** Unit tests for various scenarios, ensuring the correctness of the URL parsing logic.

- **`Cargo.toml`:** The project configuration file specifying dependencies and other metadata.

## How to Run

To run the project, execute the following command:

```bash
cargo run
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
pest = "2.1.3"
pest_derive = "2.1.3"
```

## URL Grammar
```pest
url = { scheme ~ "://" ~ host ~ (":" ~ port)? ~ (path ~ ("/" ~ file)? ~ ("?" ~ query)? ~ ("#" ~ fragment)?)? }
scheme = { identifier }
host = { identifier }
port = { number }
path = { "/" ~ identifier ~ ("/" ~ identifier)* }
file = { identifier ~ ("." ~ identifier)* }
query = { identifier ~ ("=" ~ identifier)* }
fragment = { identifier }

identifier = { ASCII_ALPHANUMERIC+ }
number = { DIGIT+ }

WHITESPACE = _{ " " | "\n" | "\t" }
ASCII_ALPHANUMERIC = _{ ASCII_ALPHABET | DIGIT }
ASCII_ALPHABET = _{ 'a'..'z' | 'A'..'Z' }
DIGIT = _{ '0'..'9' }
```

## Future Enhancements

- **Additional URL Components:** Extend the parser to support additional URL components if needed.

- **User Input Handling:** Implement a more robust user input handling mechanism, allowing users to input URLs interactively.

- **Error Messages:** Enhance error messages to provide more detailed information about parsing failures.
