use clap::Parser;
use std::collections::HashMap;

mod ast;
mod lexer;
mod parser;

use crate::ast::*;
use crate::lexer::*;
use crate::parser::*;

/// A simple program to parse a logo code file using clap.
#[derive(Parser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,
}

///
/// Main function logic:
/// - Processes command line args
/// - Initialises an Image
/// - Tokenise the logo code
/// - Parse the tokens into an AST
/// - print the parsed ast out
/// 
fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;

    // Tokenise the code
    if let Ok(tokens) = tokenise_logo(file_path) {
        let mut token_iterator = tokens.into_iter().peekable();
        let mut brackets = 0;   // this validates the start and end of each codeblock

        // Function map is used to keep track of any defined functions where the key is
        // the name of the function and value points to the AST root node of that function
        let mut function_map: HashMap<String, Node> = HashMap::new();

        // Parse the tokens
        match parse_logo(&mut token_iterator, &mut brackets, &mut function_map) {
            Ok(ast) => {
                if brackets != 0 {
                    eprintln!("Invalid codeblock");
                    return Err(());
                }
                
                // dbg print parsed ast
                dbg!(ast);
                
            }
            Err(e) => {
                eprintln!("{e}");
                return Err(());
            }
        }
    } else {
        eprintln!("Error with tokenising");
        return Err(());
    }

    Ok(())
}
