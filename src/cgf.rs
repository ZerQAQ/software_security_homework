use crate::unit;

use unit::I_token;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Node{
    pub edge: Vec<u64>,
    pub din: u64,
    pub dout: u64,
    pub id: u64,
    pub name: String,
    pub start: usize,
    pub end: usize,
}

fn is_type(string: &str) -> bool{
    if string == "char" || string == "short" || string == "int" || string == "long" 
    || string == "double" || string == "float" || string == "void"{
        return true;
    } else {
        return false;
    }
}

fn check_function_patterm(tokens: &[I_token]) -> bool{
    if let I_token::Keyword(string) = &tokens[0] {
        if !is_type(&string) {return false;}
    } else {return false;}
    if let I_token::Identifier(_) = &tokens[1]{}
    else {return false;}
    if let I_token::Operator(string) = &tokens[2]{
        if string != "(" {return false;} //左括号
    } else {return false;}
    return true;
}

pub fn scan_function(tokens: &Vec<I_token>) -> Vec<Node> {
    let mut functions: Vec<Node> = Vec::new();
    let mut functions_map: HashMap<String, usize> = HashMap::new();
    let t_len = tokens.len();
    let mut t_ptr: usize = 0;
    loop{
        if t_ptr + 3 >= t_len {break;};
        if check_function_patterm(&tokens[t_ptr..(t_ptr + 3)]) {
            let mut function_name = String::from("");
            if let I_token::Identifier(string) = &tokens[t_ptr + 1] {
                function_name = string.clone();
            }
            let mut t_ptr_n = t_ptr + 3;
            let mut num = 1;
            while num > 0 { //跳过函数参数定义
                if let I_token::Operator(string) = &tokens[t_ptr_n] {
                    if string == "(" {num += 1;}
                    if string == ")" {num -= 1;}
                }
                t_ptr_n += 1;
            }
            if let I_token::Operator(string) = &tokens[t_ptr_n] {
                if string == "{" {} 
                else {break;}
            } else {break;}
            let func_start = t_ptr_n;
            t_ptr_n += 1;
            num = 1;
            while num > 0 { //跳过函数体
                if let I_token::Operator(string) = &tokens[t_ptr_n]{
                    if string == "{" {num += 1;}
                    if string == "}" {num -= 1;}
                }
                t_ptr_n += 1;
            }
            let f = Node{
                name: function_name,
                start: func_start,
                end: t_ptr_n - 1,
                din: 0,
                dout: 0,
                edge: Vec::new(),
                id: 0,
            };
            functions.push(f);
            t_ptr = t_ptr_n;
        } else {t_ptr += 1;}
    }

    //初始化函数ID
    for (ind, func) in functions.iter_mut().enumerate() {
        functions_map.insert(func.name.clone(), ind);
        func.id = ind as u64;
    }
    // !!!
    //初始化函数调用关系
    let mut temp_din: Vec<u64> = vec![0; functions.len()];
    for func in &mut functions {
        for token in &tokens[func.start..func.end + 1]{
            if let I_token::Identifier(string) = token {
                if let Some(ind) = functions_map.get(string) {
                    func.edge.push(ind.clone() as u64);
                    func.dout += 1;
                    // clone as uszie ???
                    temp_din[ind.clone() as usize] += 1;
                    //functions[ind.clone() as usize].din += 1;
                }
            }
        }
    }
    for (ind, val) in temp_din.iter().enumerate(){
        functions[ind].din = val.clone();
    }
    // ???
    /* for func in & functions {
        for ind in & func.edge {
            functions[ind as usize].din += 1;
        }
    } */

    functions
}