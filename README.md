# AOC

Rust implementation of aoc

## How to setup
1. Put your session token into the `.aoc_token` file like below
```
session=xxxxx
```
2. Commands to run

```bash
# Run locally
cargo run <year> <day> <level> 

# Run and submit
cargo run <year> <day> <level> -s

# Run tests example
cargo test --package aoc-2025 day02
```
