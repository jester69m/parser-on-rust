use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "url.pest"]
struct UrlParser;

#[derive(Debug, Error)]
pub enum ParseUrlError {
    #[error("Error parsing URL: {0}")]
    ParsingError(String),
}

#[derive(Debug)]
pub struct ParsedUrl {
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
    pub file: Option<String>,
}

impl ParsedUrl {
    fn new() -> Self {
        ParsedUrl {
            scheme: String::new(),
            host: String::new(),
            port: None,
            path: String::new(),
            query: None,
            fragment: None,
            file: None,
        }
    }
}

pub fn parse_url(url_string: &str) -> Result<ParsedUrl, ParseUrlError> {
    let pairs = match UrlParser::parse(Rule::url, url_string) {
        Ok(pairs) => pairs,
        Err(e) => return Err(ParseUrlError::ParsingError(format!("{}", e))),
    };

    let mut parsed_url = ParsedUrl::new();

    for pair in pairs.into_iter() {
        for pair_inner in pair.into_inner() {
            match pair_inner.as_rule() {
                Rule::scheme => parsed_url.scheme = pair_inner.as_str().to_string(),
                Rule::host => parsed_url.host = pair_inner.as_str().to_string(),
                Rule::port => parsed_url.port = Some(pair_inner.as_str().parse().unwrap()),
                Rule::path => parsed_url.path = pair_inner.as_str().to_string(),
                Rule::query => parsed_url.query = Some(pair_inner.as_str().to_string()),
                Rule::fragment => parsed_url.fragment = Some(pair_inner.as_str().to_string()),
                Rule::file => parsed_url.file = Some(pair_inner.as_str().to_string()),
                _ => {
                    eprintln!("Unexpected rule: {:?}", pair_inner.as_rule());
                }
            }
        }
    }

    Ok(parsed_url)
}
