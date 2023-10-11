[![Clippy](https://github.com/nogibjj/rust-data-engineering/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/rust-data-engineering/actions/workflows/lint.yml)
[![Tests](https://github.com/nogibjj/rust-data-engineering/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/rust-data-engineering/actions/workflows/tests.yml)


# Rust Data Engineering

### Lab:  Modifying a Rust Command-Line Tool

Added a new function in cli-salad/src/lib.rs that draws a selected number of cards from the set of spades cards using a command line argument

To run: 
1. 'make all'
2. 'cd cli-salad'
3. 'cargo run -- -n 5'
Output: 
Created Fruit salad with 5 fruits: ["Fig", "Pomegranate", "Cherry", "Apple", "Loquat"]
Drew a hand with 5 spade cards: ["10 of Spades", "Jack of Spades", "7 of Spades", "5 of Spades", "Ace of Spades"]



**Goals**

This hands-on lab provides experience with:

- Forking and cloning a Rust project

- Modifying existing Rust code 

- Running `cargo build` and `cargo run`

- Version control with git

- Making a pull request (optional)


### Technical Notes

## Makefile

Each subdirectory project uses this style to make it easy to test and run

```
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run
```


## References

* [Rust Collections](https://doc.rust-lang.org/std/collections/index.html)
* [GitHub Copilot CLI](https://www.npmjs.com/package/@githubnext/github-copilot-cli)
* [Rust Fundamentals](https://github.com/alfredodeza/rust-fundamentals)
* [Rust Tutorial](https://nogibjj.github.io/rust-tutorial/)
* [Rust MLOps Template](https://github.com/nogibjj/mlops-template)
