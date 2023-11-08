#[cfg(test)]
mod tests {
   use parser_on_rust::url_parser;
   #[test]
   fn greating() {
      println!("Hello from tests from parser-on-rust");
   }

   #[test]
   fn scheme_test() {
      assert_eq!(url_parser::scheme("https://example.com/path/text.txt"), Ok("https".to_string()));
   }

   
}