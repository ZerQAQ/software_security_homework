mod lexer;
mod unit;
mod cgf;
mod stack;

extern crate crypto;

extern crate clap;
use clap::{Arg, App, SubCommand};

fn work_token(file1: &str, file2: &str){
    let lines1 = unit::get_token_lines(file1);
    let lines2 = unit::get_token_lines(file2);
    let hash1 = unit::hash_token_lines(&lines1);
    let hash2 = unit::hash_token_lines(&lines2);

    let max_len = (hash2.len() + hash1.len()) / 2;
    let ml = unit::LCS(&hash1, &hash2);
    //println!("{:?} {:?}", hash1, hash2);
    println!("there are {} token lines in {}.\nand {} token lines in {}.", hash1.len(), file1, hash2.len(), file2);
    println!("{} lines of them are similar.", ml);
    println!("the similarity is {}%", ml as f64 / max_len as f64 * 100.0);
}

fn print_cgf(vec: &Vec<cgf::Node>) {
    for func in vec {
        println!("{:?}", func);
    }
}

fn work_cgf(file1: &str, file2: &str){
    let t1 = unit::get_raw_I_token_lines(file1);
    let t2 = unit::get_raw_I_token_lines(file2);
    let func1 = cgf::scan_function(&t1);
    let func2 = cgf::scan_function(&t2);
    let condition = |x: &cgf::Node| x.din != 0 || x.dout != 0;

    let mut func1: Vec<cgf::Node> = func1.into_iter().filter(condition).collect();
    let mut func2: Vec<cgf::Node> = func2.into_iter().filter(condition).collect();

    let cmp = |a: &cgf::Node, b: &cgf::Node| {
        if a.din == b.din {return a.din.cmp(&b.din);}
        else {return a.dout.cmp(&b.dout);}
    };
    let mut f = true;
    func1.sort_unstable_by(cmp);
    func2.sort_unstable_by(cmp);
    for (ind, func) in func1.iter_mut().enumerate() {
        if func2[ind].din != func.din || func2[ind].dout != func.dout {
            f = false;
            break;
        }
    }
    //println!("{}:", file1);
    //print_cgf(&func1);
    //println!("{}:", file2);
    //print_cgf(&func2);
    if f {println!("the cgf of {} and {} is very similar", file1, file2);}
    else {println!("the cgf of {} and {} is not similar", file1, file2);}
}

fn work_stack(file: &str){
    let t = unit::get_raw_I_token_lines(file);
    let func = cgf::scan_function(&t);
    let line_info = unit::get_line_info(file);
    stack::scan_function(&func, &t, &line_info);
}
fn main() {
    let matches = App::new("code analys tool")
        .version("v0.1")
        .author("hezeyu <U201814853>")
        .about("tool to analys code, my software security homework.")
        .subcommand(SubCommand::with_name("token")
            .about("code compare base on token flow")
            .arg(Arg::with_name("filename1")
                .help("the first file you need to compare")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("filename2")
                .help("the second file you need to compare")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("cgf")
            .about("code compare base on cgf")
            .arg(Arg::with_name("filename1")
                .help("the first file you need to compare")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("filename2")
                .help("the second file you need to compare")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("check")
            .about("check code's errors")
            .arg(Arg::with_name("filename")
                .help("the file you need to check")
                .required(true)
                .takes_value(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("token") {
        let file1 = matches.value_of("filename1").unwrap();
        let file2 = matches.value_of("filename2").unwrap();
        work_token(file1, file2);
    }
    if let Some(matches) = matches.subcommand_matches("cgf") {
        let file1 = matches.value_of("filename1").unwrap();
        let file2 = matches.value_of("filename2").unwrap();
        work_cgf(file1, file2);
    }
    if let Some(matches) = matches.subcommand_matches("check") {
        let file = matches.value_of("filename").unwrap();
        work_stack(file);
    }
}
