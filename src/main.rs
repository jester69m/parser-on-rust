use std::error::Error;

use parser_on_rust::parse_url;

// fn main() {
//     let url_string = "https://www.example.com";
//     match parse_url(url_string) {
//         Ok(parsed_url) => println!("Successfully parsed URL: {:?}", parsed_url),
//         Err(err) => eprintln!("Error parsing URL: {}", err),
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    // Example URLs to parse
    let urls = [
        "https://www.example.com",
        "http://localhost:8080/path?query=value#fragment",
        "ftp://example.org",
        "https://www.example.com:8080",
        "invalid-url",
    ];

    for url in urls {
        match parse_url(url) {
            Ok(parsed_url) => {
                println!("Successfully parsed URL: {:?}", parsed_url);
            }
            Err(err) => {
                eprintln!("Error parsing URL '{}': {}", url, err);
            }
        }
    }
    Ok(())
}
