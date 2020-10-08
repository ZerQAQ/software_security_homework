use crate::cgf;
use crate::unit;

use std::collections::HashMap;
use unit::I_token;
use colored::*;

#[derive(Debug)]
struct Var {
    start: usize,
    end: usize,
    name: String,
    size: u64,
    val: String,
    tp: String,
}

fn is_type(string: &str) -> bool{
    if string == "char" || string == "short" || string == "int" || string == "long" 
    || string == "double" || string == "float" || string == "void" || string == "unsigned"{
        return true;
    } else {
        return false;
    }
}

fn check_define_pattern(tokens: &[unit::I_token]) -> bool {
    if let unit::I_token::Keyword(string) = &tokens[0] {
        if !is_type(string) {return false;}
    } else {return false;}
    if let unit::I_token::Identifier(_) = &tokens[1] {
    } else {return false;}
    if let unit::I_token::Operator(_) = &tokens[2] {
    } else {return false;}
    if let unit::I_token::Const(string) = &tokens[3] {
        let res = string.parse::<u64>();
        if let Ok(_) = res {return true;}
        else {return false;}
    } else {return false;}
}

//p = malloc(100);
//0 1 2     34  56
fn parse_define_heap(tokens: &[I_token], position: usize) -> Option<Var> {
    let mut var_name = String::from("");
    if let I_token::Identifier(string) = &tokens[0]{
        var_name = string.clone();
    } else {return None;}
    if let I_token::Operator(string) = &tokens[1]{
        if string != "=" {return None;}
    } else {return None;}
    if let I_token::Identifier(string) = &tokens[2] {
        if string != "malloc" {return None;}
    } else {return None;}
    if let I_token::Const(string) = &tokens[4] {
        if let Ok(val) = string.parse::<u64>() {
            return Some(Var{
                name: var_name,
                size: val,
                start: position,
                end: position + 7,
                val: String::from(""),
                tp: String::from(""),
            });
        } else {return None;}
    } else {return None;}
}

fn parse_define_stack(tokens: &[unit::I_token], position: usize) -> Option<Var> {
    if check_define_pattern(tokens) == false {return Option::None;}

    let mut var_name = String::from("");
    let mut var_size: u64 = 0;
    let mut val = String::from("");
    if let unit::I_token::Identifier(string) = &tokens[1] {
        var_name = string.clone();
    } 
    if let unit::I_token::Const(int) = &tokens[3] {
        var_size = int.parse::<u64>().unwrap();
    }
    if let unit::I_token::Keyword(string) = &tokens[0] {
        if string == "char" {var_size *= 1;}
        else if string == "short" {var_size *= 2;}
        else if string == "int" || string == "float" || string == "unsigned" {var_size *= 4;}
        else if string == "double" {var_size *= 8;}
    }
    if let unit::I_token::Operator(string) = &tokens[5] {
        if string == "="{
            if let unit::I_token::Const(string) = &tokens[6] {
                val = string.clone();
            }
        }
    }
    let mut end = position + 6;
    if val != "" {end += 2;}
    return Some(Var{
        name: var_name,
        size: var_size,
        start: position,
        end: end,
        val,
        tp: String::from(""),
    })
}

/* 
1     234 56 7  89
memset(a, 0, 100);
memcpy(a, b, 200); */

#[derive(Debug)]
struct Usage{
    name: String,
    size: u64,
    start: usize,
    end: usize,
}

//memcpy(a, b, 1000);
//0     123 45 6   78
fn parse_usage(tokens: &[I_token], position: usize) -> Option<Usage> {
    let mut var_string = String::from("");
    let mut var_size: u64 = 0;
    if let I_token::Identifier(string) = &tokens[0]{
        if string == "memset" || string == "memcpy" ||
        string == "memnset" || string == "memncpy" {
            if let I_token::Identifier(string) = &tokens[2] {
                var_string = string.clone();
            }
            if let I_token::Const(int) = &tokens[6] {
                var_size = int.parse::<u64>().unwrap();
            }
        } else {return Option::None;}
    } else {return Option::None;}
    return Some(Usage{
        name: var_string,
        size: var_size,
        start: position,
        end: position + 9,
    })
}

//int x;
//0   12
//int x[100];
//0   123  45
fn parse_define_single(tokens: &[I_token], position: usize) -> Option<Var>{
    let mut var_type;
    if let I_token::Keyword(string) = &tokens[0]{
        if !is_type(string) {return None;}
        var_type = string.clone();
    } else {return None;}
    let mut var_name;
    if let I_token::Identifier(string) = &tokens[1]{
        var_name = string.clone();
    } else {return None;}
    if let I_token::Operator(string) = &tokens[2]{
        if string == "[" {return None;}
    }
    let mut var_size: u64 = 1;
    if var_type == "short" {var_size = 2;}
    else if var_type == "int" || var_type == "unsigned" || var_type == "float" {var_size = 4;}
    else if var_type == "double" {var_size = 8;}
    return Some(Var{
        name: var_name,
        size: var_size,
        tp: var_type,
        start: position,
        end: position + 3,
        val: String::from(""),
    });
}

// a = b;
// 0 1 23
fn parse_usage_signal(tokens: &[I_token], position: usize) -> Option<(String, String)>{
    let mut left: String;
    if let I_token::Identifier(string) = &tokens[0] {
        left = string.clone();
    } else {return None;}
    if let I_token::Operator(string) = &tokens[1] {
        if string != "=" {return None;}
    } else {return None;}
    let mut right: String;
    if let I_token::Identifier(string) = &tokens[2] {
        right = string.clone();
    } else {return None;}
    if let I_token::Operator(string) = &tokens[3] {
        if string != ";" {return None;}
    } else {return None;}
    return Some((left, right));
}

pub fn scan_function(nodes: &Vec<cgf::Node>, tokens: &Vec<unit::I_token>, line_info: &Vec<u64>){
    let mut vars_stack: HashMap<String, Var> = HashMap::new();
    let mut vars_heap: HashMap<String, Var> = HashMap::new();
    let mut vars_single: HashMap<String, Var> = HashMap::new();
    let mut unsafe_num = 0;

    // parse var define
    for func in nodes {
        //println!("{}", func.name);
        for ind in func.start..(func.end - 6) {
            if let Some(var_object) = parse_define_stack(&tokens[ind..(ind + 7)], ind) {
                if (var_object.size + 4) < var_object.val.len() as u64 {
                    println!("----------------------------------------------------------------------");
                    println!("{} {}{}", "in function".bright_purple(), func.name.bright_red(), ":\n".bright_purple());
                    println!("{} {}", var_object.name.bright_red(), "is unsafely defined here:".yellow());
                    print!("{} {} {}", "line".cyan(), line_info[var_object.start].to_string().cyan(), ":".cyan());
                    for elm in &tokens[var_object.start..var_object.end] {print!("{} ", elm);}
                    print!("\n\n{}", "error: stack overflow".yellow());
                    print!("\n\n");
                    unsafe_num += 1;
                }

                vars_stack.insert(var_object.name.clone(), var_object);
            }
            if let Some(var_object) = parse_define_heap(&tokens[ind..(ind + 7)], ind){
                vars_heap.insert(var_object.name.clone(), var_object);
            }
        }
        for ind in func.start..(func.end - 2) {
            if let Some(var_object) = parse_define_single(&tokens[ind..(ind + 3)], ind){
                vars_single.insert(var_object.name.clone(), var_object);
            }
        }
    }

    for func in nodes {
        for ind in func.start..(func.end - 8){
            let ret = parse_usage(&tokens[ind..(ind + 9)], ind);
            //for elm in &tokens[ind..(ind + 9)] {print!("{:?} ", elm);} println!("");
            if let Some(usage_val) = ret {
                //println!("{:?}", usage_val);
                if let Some(var) = vars_stack.get(&usage_val.name) {
                    if var.size < usage_val.size {
                        println!("----------------------------------------------------------------------");
                        println!("{} {}{}", "in function".bright_purple(), func.name.bright_red(), ":\n".bright_purple());
                        println!("{} {}", var.name.bright_red(), "is defined here:".yellow());
                        print!("{} {}{} ", "line".cyan(), line_info[var.start].to_string().cyan(), ":".cyan());
                        for elm in &tokens[var.start..var.end] {print!("{} ", elm);}
                        print!("\n\n");
                        println!("{}", "and is used unsafely here:".yellow());
                        print!("{} {}{} ", "line".cyan(), line_info[usage_val.start].to_string().cyan(), ":".cyan());
                        for elm in &tokens[usage_val.start..usage_val.end] {print!("{} ", elm);}
                        print!("{}", "\n\nerror: stack overflow".yellow());
                        print!("\n\n");
                        unsafe_num += 1;
                    }
                }
                if let Some(var) = vars_heap.get(&usage_val.name) {
                    if var.size < usage_val.size {
                        println!("----------------------------------------------------------------------");
                        println!("{} {}{}", "in function".bright_purple(), func.name.bright_red(), ":\n".bright_purple());
                        println!("{} {}", var.name.bright_red(), "is defined here:".yellow());
                        print!("{} {}{} ", "line".cyan(), line_info[var.start].to_string().cyan(), ":".cyan());
                        for elm in &tokens[var.start..var.end] {print!("{} ", elm);}
                        print!("\n\n");
                        println!("{}", "and is used unsafely here:".yellow());
                        print!("{} {}{} ", "line".cyan(), line_info[usage_val.start].to_string().cyan(), ":".cyan());
                        for elm in &tokens[usage_val.start..usage_val.end] {print!("{} ", elm);}
                        print!("{}", "\n\nerror: heap overflow".yellow());
                        print!("\n\n");
                        unsafe_num += 1;
                    }
                }
            }
        }
        for ind in func.start..(func.end - 3) {
            if let Some((left, right)) = parse_usage_signal(&tokens[ind..(ind + 4)], ind) {
                let left = vars_single.get(&left).unwrap();
                let right = vars_single.get(&right).unwrap();
                let mut err_msg = String::from("null");
                if left.size < right.size {
                    err_msg = String::from("error: the left var is small than the right");
                } else if left.tp == "unsigned" && right.tp != "unsigned" {
                    err_msg = String::from("error: the left var is unsigned but the right is not");
                } else if right.tp == "unsigned" && left.tp != "unsigned" {
                    err_msg = String::from("error: the right var is unsigned but th left is not");
                }
                if err_msg != "null" {
                    unsafe_num += 1;
                    println!("----------------------------------------------------------------------");
                    println!("{} {}{}", "in function".bright_purple(), func.name.bright_red(), ":\n".bright_purple());

                    println!("{} {}", left.name.bright_red(), "is defined here:".yellow());
                    print!("{} {}{} ", "line".cyan(), line_info[left.start].to_string().cyan(), ":".cyan());
                    for elm in &tokens[left.start..left.end] {print!("{} ", elm);} 

                    println!("\n\n{} {}", right.name.bright_red(), "is defined here:".yellow());
                    print!("{} {}{} ", "line".cyan(), line_info[right.start].to_string().cyan(), ":".cyan());
                    for elm in &tokens[right.start..right.end] {print!("{} ", elm);}
                    println!("\n\n{}", "unsafe usage is found here:".yellow());
                    for elm in &tokens[ind..(ind + 4)] {print!("{} ", elm);}
                    println!("\n\n{}", err_msg.yellow());
                }
            } 
        }
    }
    println!("----------------------------------------------------------------------");
    println!("{} {}", unsafe_num.to_string().yellow(), "unsafe use is found".yellow());
}