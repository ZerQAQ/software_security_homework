/*
    基于Token和LCS算法来分析相似度
*/

use crate::lexer;

use lexer::Lexer;
use lexer::Token;
use std::fs;

extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::cmp;
use std::collections::HashMap;

fn filt_separator(l: &mut Lexer) {
    loop {
        let t = l.touch_next_token().unwrap();
        if let Token::Operator(code) = t {
            if code == 34 || code == 35 || code == 28 {
                l.flash_pointer();
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn read_line(l: &mut Lexer) -> Vec<Token> {
    let mut ret: Vec<Token> = Vec::new();
    loop {
        if l.is_EOF() {
            break;
        }
        let t = l.touch_next_token().unwrap();
        if let Token::Operator(code) = t {
            if code == 34 || code == 35 || code == 28 {
                break;
            } else {
                ret.push(t);
                l.flash_pointer();
            }
        } else {
            ret.push(t);
            l.flash_pointer();
        }
    }
    return ret;
}

/* fn print_token_lines(lines: &Vec<Vec<Token>>, l: &Lexer) {
    for line in lines {
        for token in line {
            print_token(l, &token);
        }
        println!("");
    }
} */

pub fn hash_token_lines(lines: &Vec<Vec<Token>>) -> Vec<String> {
    let mut sh = Md5::new();
    let mut ret: Vec<String> = Vec::new();
    for line in lines {
        sh.reset();
        for token in line {
            match token {
                Token::Operator(code) => sh.input_str(&format!("OP_{} ", code)),
                Token::Keyword(code) => sh.input_str(&format!("KW_{} ", code)),
                Token::Identifier(_) => sh.input_str("ID "),
                Token::Const(s) => sh.input_str(&format!("CST_{} ", s)),
                _ => {}
            }
        }
        let s = sh.result_str();
        ret.push(s);
    }
    return ret;
}

//???
#[derive(Debug)]
pub enum I_token {
    Keyword(String),
    Identifier(String),
    Const(String),
    Operator(String),
    EOF,
    Endl,
}

use std::fmt;

impl fmt::Display for I_token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            I_token::Keyword(s) => f.write_str(&s),
            I_token::Identifier(s) => f.write_str(&s),
            I_token::Const(s) => f.write_str(&s),
            I_token::Operator(s) => f.write_str(&s),
            I_token::Endl => f.write_str("\n"),
            I_token::EOF => f.write_str("EOF"),
        }
    }
}

fn token_to_I(token: Token, lexer: &Lexer) -> I_token {
    match token {
        Token::Keyword(code) => I_token::Keyword(lexer.keyword_map_f.get(&code).unwrap().clone()),
        Token::Identifier(code) => {
            I_token::Identifier(lexer.identifier_map_f.get(&code).unwrap().clone())
        }
        Token::Const(string) => I_token::Const(string.clone()),
        Token::Operator(code) => {
            I_token::Operator(lexer.operator_map_f.get(&code).unwrap().clone())
        }
        Token::EOF => I_token::EOF,
        Token::Endl => I_token::Endl,
    }
}

pub fn get_raw_I_token_lines_flag(file_name: &str, endl_flag: bool) -> Vec<I_token> {
    let s = fs::read_to_string(file_name).unwrap();
    let mut l = Lexer::new(&s);
    l.endl_flag = endl_flag;

    let mut tokens: Vec<Token> = Vec::new();
    loop {
        if l.is_EOF() {
            break;
        }
        tokens.push(l.get_next_token().unwrap());
    }
    parse_typedef_raw(&mut tokens);

    let ret: Vec<I_token> = tokens.into_iter().map(|x| token_to_I(x, &l)).collect();

    ret
}

pub fn get_raw_I_token_lines(file_name: &str) -> Vec<I_token> {
    get_raw_I_token_lines_flag(file_name, false)
}

pub fn get_token_lines(file_name: &str) -> Vec<Vec<Token>> {
    let s = fs::read_to_string(file_name).unwrap();
    let mut l = Lexer::new(&s);

    let mut lines: Vec<Vec<Token>> = Vec::new();
    loop {
        filt_separator(&mut l);
        let line = read_line(&mut l);
        lines.push(line);
        if l.is_EOF() {
            break;
        }
    }
    parse_typedef(&mut lines);
    lines
}

pub fn get_line_info(file_name: &str) -> Vec<u64> {
    let tokens = get_raw_I_token_lines_flag(file_name, true);
    let mut ret: Vec<u64> = Vec::new();
    let mut line = 1;
    for elm in &tokens {
        if let I_token::Endl = elm {
            line += 1;
        } else {
            ret.push(line);
        }
    }
    ret
}

pub fn LCS<T: std::cmp::PartialEq>(s1: &Vec<T>, s2: &Vec<T>) -> u32 {
    let len = s1.len() * s2.len();
    let mut d = vec![0u32; len];

    let len1 = s1.len();
    let len2 = s2.len();
    if s1[0] == s2[0] {
        d[0] = 1;
    }
    for i in 0..len1 {
        for j in 0..len2 {
            let ind = i * len2 + j;
            if i != 0 {
                d[ind] = cmp::max(d[ind], d[ind - len2]);
            }
            if j != 0 {
                d[ind] = cmp::max(d[ind], d[ind - 1]);
            }
            if i != 0 && j != 0 && s1[i] == s2[j] {
                d[ind] = cmp::max(d[ind], d[ind - len2 - 1] + 1);
            }
        }
    }
    return d[len - 1];
}

fn parse_typedef_raw(line: &mut Vec<Token>) {
    let mut m = HashMap::new();
    for (i, token) in line.iter().enumerate() {
        // token = KW_typedef
        if let Token::Keyword(3) = token {
            if let Token::Keyword(kw_code) = line[i + 1] {
                if let Token::Identifier(id_code) = line[i + 2] {
                    m.insert(id_code, kw_code);
                }
            }
        }
    }
    for token in line {
        if let Token::Identifier(id_code) = token {
            if let Some(kw_code) = m.get(id_code) {
                *token = Token::Keyword(kw_code.clone());
            }
        }
    }
}

fn parse_typedef(lines: &mut Vec<Vec<Token>>) {
    let mut m = HashMap::new();
    for line in &(*lines) {
        for (i, token) in line.iter().enumerate() {
            // token = KW_typedef
            if let Token::Keyword(3) = token {
                if let Token::Keyword(kw_code) = line[i + 1] {
                    if let Token::Identifier(id_code) = line[i + 2] {
                        m.insert(id_code, kw_code);
                    }
                }
            }
        }
    }
    for line in lines {
        for token in line {
            if let Token::Identifier(id_code) = token {
                if let Some(kw_code) = m.get(id_code) {
                    *token = Token::Keyword(kw_code.clone());
                }
            }
        }
    }
}
