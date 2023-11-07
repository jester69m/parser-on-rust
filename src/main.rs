use parser_on_rust::scheme;
pub fn main() {
    let input = "https://example.com/path";

    match scheme(input) {
        Ok(scheme) => {
            println!("Scheme: {}", scheme);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}