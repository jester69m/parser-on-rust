use parser_on_rust::url_parser;
pub fn main() {
    let input = "https://example.com/path";
    
    match url_parser::scheme(input) {
        Ok(scheme) => {
            println!("Scheme: {}", scheme);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}