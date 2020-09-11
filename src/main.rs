mod lexer;

use lexer::print_token;
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

fn print_token_lines(lines: &Vec<Vec<Token>>, l: &Lexer) {
    for line in lines {
        for token in line {
            print_token(l, &token);
        }
        println!("");
    }
}

macro_rules! hash_token_unit {
    ($($type:expr, $fmt:expr),*) => {
        $( Token::$type(code) => sh.input_str(&format!($fmt, code)), )*
    };
}

fn hash_token_lines(lines: &Vec<Vec<Token>>) -> Vec<String> {
    let mut sh = Md5::new();
    let mut ret: Vec<String> = Vec::new();
    for line in lines {
        sh.reset();
        for token in line {
            match token {
                Token::Operator(code) => sh.input_str(&format!("OP_{} ", code)),
                Token::Keyword(code) => sh.input_str(&format!("KW_{} ", code)),
                Token::Identifier(code) => sh.input_str("ID "),
                Token::Const(s) => sh.input_str(&format!("CST_{} ", s)),
                _ => {}
            }
        }
        let s = sh.result_str();
        ret.push(s);
    }
    return ret;
}

fn get_token_lines(file_name: &str) -> Vec<Vec<Token>> {
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

    lines
}

fn LCS<T: std::cmp::PartialEq>(s1: &Vec<T>, s2: &Vec<T>) -> u32 {
    let len = s1.len() * s2.len();
    let mut d = vec![0u32; len];

    let len1 = s1.len();
    let len2 = s2.len();
    if s1[0] == s2[0] {d[0] = 1;}
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

fn parse_typedef(mut lines: &mut Vec<Vec<Token>>){
    let mut m = HashMap::new();
    for line in &(*lines) {
        for (i, token) in line.iter().enumerate() {
            // token = KW_typedef
            if let Token::Keyword(3) = token {
                if let Token::Keyword(kw_code) = line[i + 1] {
                    if let Token::Identifier(id_code) = line[i + 2]{
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

extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {

    let matches = App::new("code compare tool")
        .version("0.1")
        .author("hezeyu <U201814853>")
        .about("tool to compare similar code, my software security homework.")
        .subcommand(SubCommand::with_name("token")
            .about("code compare base on token flow")
            .arg(Arg::with_name("f1")
                .help("the first file you need to compare")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("f2")
                .help("the second file you need to compare")
                .required(true)
                .takes_value(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("token") {
        let file_1 = matches.value_of("f1").unwrap();
        let file_2 = matches.value_of("f2").unwrap();

        let mut lines1 = get_token_lines(file_1);
        let mut lines2 = get_token_lines(file_2);
        parse_typedef(&mut lines1);
        parse_typedef(&mut lines2);
        let hash1 = hash_token_lines(&lines1);
        let hash2 = hash_token_lines(&lines2);

        let max_len = (hash2.len() + hash1.len()) / 2;
        let ml = LCS(&hash1, &hash2);
        //println!("{:?} {:?}", hash1, hash2);
        println!("there are {} token lines in {}.\nand {} token lines in {}.", hash1.len(), file_1, hash2.len(), file_2);
        println!("{} lines of them are similar.", ml);
        println!("the similarity is {}%", ml as f64 / max_len as f64 * 100.0);
    }
}
