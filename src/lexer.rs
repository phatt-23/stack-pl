use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use crate::keyword::KeywordType;
use crate::token::Token;
use crate::location::Location;
use crate::intrinsic::IntrinsicType;

pub fn lex_file_to_tokens(file: &str) -> Result<Vec<Token>, io::Error> {
    // println!("[INFO]: reading the program from '{}'", file);
    let lines = read_lines(file)?;
    Ok(
        lines.enumerate()
        .flat_map(|(row, line)| lex_line_to_tokens(line.unwrap(), file, row))
        .collect()
    )
}

fn lex_line_to_tokens(line: String, file: &str, row: usize) -> Vec<Token>
{
    let line = line.split_at(
        line.find("//").unwrap_or(line.len())
    ).0.to_string();

    let mut col: usize = find_col(&line, 0, |x| x != b' ');
    let mut col_end: usize;
    let mut tokens: Vec<Token> = Vec::new();
    
    while col < line.len() {
        let loc = Location::new(file, row, col);
        let bline = line.as_bytes();
        if bline[col] == b'"' { // Parsing String Literal
            col_end = find_col(&line, col + 1, |x| x == b'"');
            assert!(bline[col] == b'"');
            assert!(col_end < bline.len() && bline[col_end] == b'"', "[ERROR]: {} Found double quote starting a string without enclosing double quote!", loc);

            let value = &line[(col + 1)..col_end].to_string();
            tokens.push(Token::new_string(value, &loc));

            col = find_col(&line, col_end + 1, |x| x != b' ');
        } else if bline[col] == b'\'' { // Parse Char
            col_end = find_col(&line, col + 1, |x| x == b'\'');
            assert!(bline[col] == b'\'');
            assert!(col_end < bline.len() && bline[col_end] == b'\'', "[ERROR]: {} Found single quote starting a string without enclosing single quote!", loc);

            let value = &line[(col + 1)..(col_end)].to_string();

            match value.as_str() {
                "\\n" => tokens.push(Token::new_char('\n', &loc)),
                "\\t" => tokens.push(Token::new_char('\t', &loc)),
                "\\r" => tokens.push(Token::new_char('\r', &loc)),
                "\\0" => tokens.push(Token::new_char('\0', &loc)),
                _ if value.len() > 1 => panic!("[ERROR]: {} '{}' is an invalid Char type, must be a single character!", &loc, value),
                s => {
                    let c = s.chars().next().unwrap();
                    tokens.push(Token::new_char(c, &loc));
                }
            }

            col = find_col(&line, col_end + 1, |x| x != b' ');
        } else { // Parse Everything Else
            col_end = find_col(&line, col, |x| x == b' ');
            let word = &line[col..col_end]; 

            if let Some(keyword) = KeywordType::from_str(word) {
                tokens.push(Token::new_keyword(keyword, &loc));
            } else if let Some(intrinsic) = IntrinsicType::from_str(word) {
                tokens.push(Token::new_intrinsic(intrinsic, &loc));
            } else if let Ok(integer) = word.parse::<i32>() {
                tokens.push(Token::new_integer32(integer, &loc));
            } else if let Ok(integer) = word.parse::<i64>() {
                tokens.push(Token::new_integer64(integer, &loc));
            } else if let Ok(integer) = word.parse::<i128>() {
                panic!("[ERROR]: {} {} Integer128 no supported!", loc, integer);
            } else {
                tokens.push(Token::new_word(&word.to_string(), &loc));
            }

            col = find_col(&line, col_end, |x| x != b' ');
        }
        
    }

    tokens
}

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>, io::Error>
where P: AsRef<std::path::Path> 
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
