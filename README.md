# parser-on-rust
Education purpose to implement parser for URL
## Idea
Take url and parse it to several element like :
- Protocol
- Host
- Subdomain
- Domain
- Resource
- Path
- File suffix
## Process
Use [peg](https://docs.rs/peg/latest/peg/)
Parser takes string then find all possible elements from it and return
## Usage
This can be usefull for programs that work with url
```rust
```
