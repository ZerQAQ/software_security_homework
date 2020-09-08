mod lexer;

use lexer::Lexer;
use lexer::Token;
use std::fs;

fn main() {
    let s = fs::read_to_string("test.c").unwrap();
    
    let mut l = Lexer::new(&s);
    loop{
        let v = l.next_token().unwrap();
        match v {
            Token::Identifier(code) => {
                print!("ID_{} ", l.identifier_map_f.get(&code).unwrap());
            },Token::Keyword(code) => {
                print!("KW_{} ", l.keyword_map_f.get(&code).unwrap());
            },Token::EOF => {
                print!("EOF");
                break;
            },Token::Operator(code) => {
                print!("OP_{} ", l.operator_map_f.get(&code).unwrap());
            },Token::Const(s) => {
                print!("CST_{} ", s);
            },Token::Endl => {
                print!("\n");
            }
        }
    }
}
