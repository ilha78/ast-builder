# ast-builder: Lexer and Parser for Logo

This project was written in Rust to parse Logo code into an Abstract Syntax Tree.

## Setup

Clone this repository and run `cargo build` to compile.

## How to Use

Run `cargo run <pathname>` where `pathname` is a file containing Logo code.

Note that the parsed Abstract Syntax Tree will be printed to stdout.

## Lexer
lexer.rs is responsible for reading the Logo code and returning a vector of each word that are split by whitespace

## Parser
parser.rs is responsible for taking the vector of words and then parsing into an Abstract Syntax Tree.

## Abstract Syntax Tree
ast.rs contains all the enums used to represent the nodes in the AST. Notably, there are two main expressions: Unary and Binary.
