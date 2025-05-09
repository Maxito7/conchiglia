use std::path::Path;
use std::process::Command;
use std::{env, io};

// Errors defined for parsing
#[derive(Debug)]
pub enum ParseError {
    QuoteMismatch,
}

pub fn parse_command(line: &str) -> Result<Vec<String>, ParseError> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut quotes_content: Option<char> = None;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match (quotes_content, c) {
            // If the condition is met, that means we have encountered the closing quote
            (Some(q), _) if c == q => {
                quotes_content = None;
            }
            // Inside of a quoted section, so we add 'c' to our current token
            (Some(_), _) => {
                current_token.push(c);
            }
            (None, '\'') | (None, '"') => quotes_content = Some(c),
            (None, c) if c.is_whitespace() => {
                if !current_token.is_empty() {
                    tokens.push(current_token);
                    current_token = String::new();
                } else {
                    tokens.push("".to_string());
                    current_token = String::new();
                }
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_whitespace() {
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
            (None, _) => {
                current_token.push(c);
            }
        }
    }

    if quotes_content.is_some() {
        return Err(ParseError::QuoteMismatch);
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    } else {
        tokens.push("".to_string());
    }

    Ok(tokens)
}

// Special function to handle built-in `cd`
pub fn handle_cd(tokens: Vec<String>) {
    if tokens.len() > 2 {
        eprintln!(
            "error: cd requires exactly one argument if you want to go to an specific directory"
        );
        return;
    }

    if tokens.len() == 1 {
        let root = Path::new("/");
        let _ = env::set_current_dir(root);
    } else {
        let err_cond = env::set_current_dir(&tokens[1]);
        match err_cond {
            Ok(_) => eprintln!(),
            Err(e) => eprintln!("{}", e),
        }
    }
}

// General function to run system-wide commands
pub fn run_command(tokens: Vec<String>) {
    let mut cmd = Command::new(&tokens[0]);
    cmd.args(&tokens[1..]);

    match cmd.status() {
        Ok(status) => {
            if let Some(code) = status.code() {
                if code != 0 {
                    eprintln!("error: command exited with code {}", code);
                }
            } else {
                eprintln!("error: command terminated by signal");
            }
        }
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                eprintln!("error: command not found: {}", tokens[0]);
            } else {
                eprintln!("error: {}", e);
            }
        }
    }
}

pub fn process_line(line: &str) {
    let trimmed_line = line.trim();
    if trimmed_line.is_empty() {
        return;
    }

    match parse_command(trimmed_line) {
        Ok(tokens) => {
            if tokens.is_empty() {
                return;
            }

            match tokens[0].as_str() {
                "cd" => handle_cd(tokens),
                "exit" => std::process::exit(0),
                _ => run_command(tokens),
            }
        }
        Err(ParseError::QuoteMismatch) => {
            eprintln!("error: mismatched quotes");
        }
    }
}
