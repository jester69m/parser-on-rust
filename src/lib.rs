peg::parser!{
    pub grammar url_parser() for str {
      rule number() -> u32
        = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }
  
      pub rule list() -> Vec<u32>
        = "[" l:(number() ** ",") "]" { l }
    
      #[no_eof]
      pub rule scheme() -> String
        = s:$(['a'..='z' | '0'..='9' | '-' | '.']+) ":" { s.to_string() }
    }
}