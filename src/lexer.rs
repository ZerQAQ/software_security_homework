use std::collections::HashMap;
use std::error::Error;
#[derive(Debug)]
pub enum Token {
    Keyword(u32),
    Identifier(u32),
    Const(String),
    Operator(u32),
    Endl,
    EOF,
} 
pub struct Lexer<'a> {
    pub keyword_map: HashMap<String, u32>,
    pub keyword_map_f: HashMap<u32, String>,

    pub operator_map: HashMap<String, u32>,
    pub operator_map_f: HashMap<u32, String>,

    pub identifier_map: HashMap<String, u32>,
    pub identifier_map_f: HashMap<u32, String>,
    id_num: u32,

    pub endl_flag: bool,
    text: &'a str,
    text_bytes: &'a [u8],
    text_len: usize,
    pointer: usize,
    pointer_next: usize,
}
macro_rules! map {
    ($new_func:expr, $($value:expr),*) => {
        {
            let temp_vec = vec![$( $new_func($value), )*];
            let mut temp_map = HashMap::new();

            for (ind, val) in temp_vec.iter().enumerate(){
                temp_map.insert(val.clone(), ind as u32);
            }
            temp_map
        }
    };
}
macro_rules! map_f {
    ($map:expr) => {
        {
        let mut temp_map = HashMap::new();
        for (key, value) in $map {
            temp_map.insert(value.clone(), key.clone());
        }
        temp_map
    }
    };
}
impl<'a> Lexer<'a> {
    pub fn new(text: &str) -> Lexer {
        let mut l = Lexer {
            keyword_map: map!( 
                String::from,
                "auto", "union",
                "break", "typedef",
                "case", "switch", "_Complex",
                "char", "struct", "_Imaginary",
                "const", "static",
                "continue", "sizeof",
                "default", "signed",
                "do", "short", "restrict",
                "double", "return", "_Bool",
                "else", "register", "inline",
                "enum", "long", "while",
                "extern", "int", "volatile",
                "float", "if", "void",
                "for", "goto", "unsigned",
                "NULL"
            ),
            keyword_map_f: HashMap::new(),
            operator_map: map!(
                String::from,
                "+", "+=", "-", "-=", "*",
                "*=", "/", "/=", "%", "%=",
                "&", "|", "^", "~", "!", ">>=", "<<=",
                "&=", "|=", "^=", "~=", ">>", "<<",
                "||", "&&", "==", "<", ">", ";",
                ">=", "<=", "++", "--", "!=", "{", "}",
                "(", ")", "[", "]", "=", "->", ",", ".",
                "//", "#"
            ),
            operator_map_f: HashMap::new(),
            identifier_map: HashMap::new(),
            identifier_map_f: HashMap::new(),
            pointer: 0,
            pointer_next: 0,
            id_num: 0,
            endl_flag: false,
            text,
            text_bytes: text.as_bytes(),
            text_len: text.len(),
        };
        l.operator_map_f = map_f!(&l.operator_map);
        l.keyword_map_f = map_f!(&l.keyword_map);
        l
    }
/*     pub fn prt(&self) {
        for (k, v) in &self.keyword_map {
            println!("{} {}", k, v);
        }
        for (k, v) in &self.keyword_map_f {
            println!("{} {}", k, v);
        }
    } */
    pub fn is_EOF(&mut self) -> bool{
        let t = self.touch_next_token().unwrap();
        if let Token::EOF = t {
            return true;
        } else {
            return false;
        }
    }
    pub fn touch_next_token(&mut self) -> Result<Token, Box<dyn Error>> {
        let pointer = self.pointer;
        let ret = self.get_next_token();
        self.pointer_next = self.pointer;
        self.pointer = pointer;
        ret
    }
    pub fn flash_pointer(&mut self){
        self.pointer = self.pointer_next;
    }
    pub fn get_next_token(&mut self) -> Result<Token, Box<dyn Error>> {
        //跳过空格符
        while self.pointer < self.text_len && is_space(self.text_bytes[self.pointer] as char) {
            if self.text_bytes[self.pointer] as char == '\n'{
                self.pointer += 1;
                if self.endl_flag{
                    return Ok(Token::Endl);
                }
            } else{
                self.pointer += 1;
            }
        }
        //如果遇到尾部返回EOF
        if self.pointer == self.text_len {
            return Ok(Token::EOF)
        }
        let first_char = self.text_bytes[self.pointer] as char;
        //println!("{}", first_char);
        if is_const(first_char) {
            //读常数
            return self.read_const();
        } else if is_id(first_char) {
            //读标识符、关键字
            return self.read_id();
        } else {
            if first_char == '.' {
                //有可能是符号也可能是数字
                if self.pointer + 1 == self.text_len {
                    return self.read_operator();
                }
                let first_char = self.text_bytes[self.text_len + 1] as char;
                if is_const(first_char) {
                    return self.read_operator();
                }
                return self.read_const();
            }
            //略过宏定义和注释
            if first_char == '#' || (self.pointer + 2 <= self.text_len && &self.text[self.pointer..self.pointer + 2] == "//"){
                let mut temp_c = ' ';
                while temp_c != '\n' {
                    temp_c = self.text_bytes[self.pointer] as char;
                    //let char_size = count_char_size(temp_c);
                    //println!("[{}] {}", temp_c, char_size);
                    self.pointer += 1;
                }
                return self.get_next_token();
            }
            //读符号
            return self.read_operator();
        }
    }
    fn get_identifier_token(&mut self, val: &str) -> Token{
        // 判断是否是关键字
        match self.keyword_map.get(val) {
            None => {
                // 如果不是
                // 判断Id是否存在于Id表，如果不存在就插入Id表并使id_num加一
                let code = self.identifier_map.entry(val.to_string()).or_insert(self.id_num);
                if *code == self.id_num {
                    self.id_num += 1;
                    self.identifier_map_f.insert(*code, val.to_string());
                }
                return Token::Identifier(code.clone());
            } ,
            Some(code) => {
                // 如果是 返回关键字Token
                return Token::Keyword(code.clone());
            }
        }
    }
    fn read_id(&mut self) -> Result<Token, Box<dyn Error>> {
        let mut end = self.pointer;
        let text_as_bytes = self.text.as_bytes();
        while end < self.text_len && is_id_content(text_as_bytes[end] as char) {
            end += 1;
        }
        let ret = Ok(self.get_identifier_token(&self.text[self.pointer..end]));
        self.pointer = end;
        ret
    }
    fn read_const(&mut self) -> Result<Token, Box<dyn Error>> {
        let first_char = self.text_bytes[self.pointer] as char;
        let mut end = self.pointer;
        if first_char == '\'' || first_char == '\"' {
            end += 1;
            while end < self.text_len && self.text_bytes[end] as char != first_char {
                end += 1;
                if self.text_bytes[end] as char == '\\' {
                    end += 2;
                }
            }
            end += 1;
        } else {
            while end < self.text_len && is_dig(self.text_bytes[end] as char){
                end += 1;
            }
        }
        let ret = Ok(Token::Const(self.text[self.pointer..end].to_string()));
        self.pointer = end;
        ret
    }
    fn read_operator(&mut self) -> Result<Token, Box<dyn Error>> {
        let mut i = 4;
        while i > 0 {
            i -= 1;
            if self.pointer + i > self.text_len {
                continue;
            }
            let operator = &self.text[self.pointer..self.pointer + i];
            if let Some(code) = self.operator_map.get(operator) {
                self.pointer += i;
                return Ok(Token::Operator(code.clone()));
            }
        }
        panic!("is a illage operator {}. in {}", &self.text[self.pointer..self.pointer + 10], self.pointer);
    }
}
#[inline(always)]
fn is_space(c: char) -> bool {
    c == ' ' || c == '\n' || c == '\t'
}
#[inline(always)]
fn is_const(c: char) -> bool{
    ('0' <= c && c <= '9') || c == '\'' || c == '\"'
}
#[inline(always)]
fn is_id(c: char) -> bool{
    ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_'
}
#[inline(always)]
fn is_id_content(c: char) -> bool{
    is_id(c) || ('0' <= c && c <= '9')
}
#[inline(always)]
fn is_dig(c: char) -> bool{
    ('0' <= c && c <= '9') || 
    ('a' <= c && c <= 'z') ||
    ('A' <= c && c <= 'Z') ||
    c == '.'
}

/* pub fn print_token(l: &Lexer, t: &Token){
    match t {
        Token::Identifier(code) => {
            print!("ID_{} ", l.identifier_map_f.get(&code).unwrap());
        },Token::Keyword(code) => {
            print!("KW_{} ", l.keyword_map_f.get(&code).unwrap());
        },Token::EOF => {
            print!("EOF");
        },Token::Operator(code) => {
            print!("OP_{} ", l.operator_map_f.get(&code).unwrap());
        },Token::Const(s) => {
            print!("CST_{} ", s);
        },Token::Endl => {
            print!("\n");
        }
    }
} */

/* fn count_char_size(c: char) -> usize {
    let c = c as u32;
    for i in (1..4) {
        if c < (1 << (i * 8)) {return i as usize};
    }
    return 4;
} */