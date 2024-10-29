use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

type LexerResult<T> = Result<T, String>;

///
/// This function is responsible for tokenising the logo code
/// For each line, it will split by whitespace
/// Then it considers any words with prefix of a literal (") or
/// variable (:), and creates a whitespace between the prefix and
/// the name and stores it in the output Vec<String>
/// A newline String is also added at the end of each line to help
/// validating code when parsing
/// 
pub fn tokenise_logo<P>(filename: P) -> LexerResult<Vec<String>>
where
    P: AsRef<Path>,
{
    let mut tokens = Vec::new();

    if let Ok(lines) = lines_from_file(filename) {
        for line in lines {
            let split_line: Vec<&str> = line.split_whitespace().collect();
            for word in split_line {
                if word.starts_with('\"') {
                    tokens.push(String::from("\""));
                    let res = remove_prefix(word);
                    match res {
                        Some(remainder) => tokens.push(remainder.to_string()),
                        None => return Err(String::from("Could not remove prefix")),
                    }
                } else if word.starts_with(':') {
                    tokens.push(String::from(":"));
                    let res = remove_prefix(word);
                    match res {
                        Some(remainder) => tokens.push(remainder.to_string()),
                        None => return Err(String::from("Could not remove prefix")),
                    }
                } else {
                    tokens.push(word.to_string());
                }
            }
            tokens.push(String::from("\n"));
        }
    } else {
        return Err(String::from("File does not exist"));
    }

    Ok(tokens)
}

///
/// Returns a vector of lines in the given file
/// 
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

///
/// Removes the specified prefix and returns the remaining String
/// 
fn remove_prefix(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

