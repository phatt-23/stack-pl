use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use crate::operation::{OperationType, Operation};
use crate::token::{Token, TokenKind};

pub fn lex_file_to_tokens(file: &str) -> Vec<Token> {
    // println!("[INFO]: reading the program from '{}'", file);
    let lines = read_lines(file);
    return lines
        .enumerate()
        .flat_map(|(row, line)| lex_line_to_tokens(line.unwrap(), file, row))
        .collect();
}

fn lex_line_to_tokens(line: String, file: &str, row: usize) -> Vec<Token>
{
    // println!("[INFO]: Original ({row}): {:?}", line);
    let line = line
        .split_at(line.find("//")
        .unwrap_or_else(|| line.len()))
        .0.to_string();
    // println!("[INFO]: UnComment ({row}): {:?}", line);

    let mut col: usize = find_col(&line, 0, |x| x != b' ');
    let mut col_end: usize;
    let mut tokens: Vec<Token> = Vec::new();
    
    while col < line.len() {
        let tok: Token;
        if line.as_bytes()[col] == b'"' { // Parsing String literal
            col_end = find_col(&line, col + 1, |x| x == b'"');
            assert!(line.as_bytes()[col] == b'"');
            assert!(line.as_bytes()[col_end] == b'"');

            let value = &line[(col + 1)..=(col_end - 1)].to_string();
            tok = Token::new_string(value, &file.to_string(), row, col);

            col = find_col(&line, col_end + 1, |x| x != b' ');
        } else {
            col_end = find_col(&line, col, |x| x == b' ');

            let word = &line[col..col_end]; 
            tok = match word.parse::<i32>() {
                Ok(v)  => Token::new_integer(v, &file.to_string(), row, col),
                Err(_) => Token::new_word(&word.to_string(), &file.to_string(), row, col),
            };
            
            col = find_col(&line, col_end, |x| x != b' ');
        }
        tokens.push(tok);
    }

    return tokens;
}



fn read_lines<P>(filename: P) 
    -> io::Lines<io::BufReader<File>>
where P: AsRef<std::path::Path> 
{
    let file = File::open(filename).unwrap_or_else(|e| panic!("[ERROR]: {e}") );
    io::BufReader::new(file).lines()
}

fn find_col(line: &String, index: usize, find: impl Fn(u8) -> bool) -> usize {
    let mut i = index;
    while i < line.len() && !find(line.as_bytes()[i]) {
        if i >= line.len() {
            panic!("Reached the end of the line without satisfying the predicate");
        }
        i += 1;
    }
    i
}
