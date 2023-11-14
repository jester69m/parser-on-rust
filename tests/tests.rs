use parser_on_rust::parse_url;

#[test]
fn test_parse_url_with_scheme_and_host_only() {
    let url = "https://www.example.com";
    let parsed_url = parse_url(url).expect("Failed to parse URL");

    assert_eq!(parsed_url.scheme, "https");
    assert_eq!(parsed_url.host, "www.example.com");
    assert_eq!(parsed_url.port, None);
    assert_eq!(parsed_url.path, "");
    assert_eq!(parsed_url.query, None);
    assert_eq!(parsed_url.fragment, None);
    assert_eq!(parsed_url.file, None);
}

#[test]
fn test_parse_url_with_all_elements_except_file() {
    let url = "http://localhost:8080/path/to/resource?query=value#fragment";
    let parsed_url = parse_url(url).expect("Failed to parse URL");

    assert_eq!(parsed_url.scheme, "http");
    assert_eq!(parsed_url.host, "localhost");
    assert_eq!(parsed_url.port, Some(8080));
    assert_eq!(parsed_url.path, "/path/to/resource");
    assert_eq!(parsed_url.query, Some("query=value".to_string()));
    assert_eq!(parsed_url.fragment, Some("fragment".to_string()));
    assert_eq!(parsed_url.file, None);
}

#[test]
fn test_parse_url_without_file() {
    let url = "ftp://example.org";
    let parsed_url = parse_url(url).expect("Failed to parse URL");

    assert_eq!(parsed_url.scheme, "ftp");
    assert_eq!(parsed_url.host, "example.org");
    assert_eq!(parsed_url.port, None);
    assert_eq!(parsed_url.path, "");
    assert_eq!(parsed_url.query, None);
    assert_eq!(parsed_url.fragment, None);
    assert_eq!(parsed_url.file, None);
}

#[test]
fn test_parse_url_with_scheme_host_and_port() {
    let url = "https://www.example.com:8080";
    let parsed_url = parse_url(url).expect("Failed to parse URL");

    assert_eq!(parsed_url.scheme, "https");
    assert_eq!(parsed_url.host, "www.example.com");
    assert_eq!(parsed_url.port, Some(8080));
    assert_eq!(parsed_url.path, "");
    assert_eq!(parsed_url.query, None);
    assert_eq!(parsed_url.fragment, None);
    assert_eq!(parsed_url.file, None);
}

#[test]
fn test_parse_url_exception() {
    // An invalid URL that should result in an error
    let url = "invalid-url";
    assert!(parse_url(url).is_err());
}
