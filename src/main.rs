extern crate rustcc;
use rustcc::*;

use std::env;
use std::io::Write;
use std::fs::File;
use std::io::prelude::*;

mod token;
mod parse;
mod gen;

use token::*;
use parse::*;
use gen::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("invalid arguments");
        return;
    }

    let mut is_print_contents = false;
    let mut is_print_asm = false;
    let mut is_file = false;
    let mut input = String::new();
    for arg in args[1..].iter() {
        // cargo runでは "-" 始まりの引数を渡せないためエスケープ用の文字を追加
        let mut arg: &str = arg;
        if &arg[0..1] == "^" {
            arg = &arg[1..];
        }
        match arg {
            "-i" => is_print_contents = true,
            "-a" => is_print_asm = true,
            "-f" => is_file = true,
            _ => input = arg.to_string()
        }
    }

    compile(input, is_file, is_print_contents, is_print_asm);
}

fn compile(input: String, is_file: bool, is_print_contents: bool, is_print_asm: bool) {
    let mut contents = String::new();
    if is_file {
        let mut f = File::open(input).expect("file not found");
        f.read_to_string(&mut contents).expect("failure read file");
    } else {
        contents = input;
    }

    let tokens = tokenize(contents.as_str());
    let nodes = parse(&tokens);
    let asm = gen_x86(nodes);

    if is_print_contents {
        println!("------- read contents -------");
        println!("{}", contents);
        println!("-----------------------------\n");
    }
    if is_print_asm {
        println!("------- output assembly -------");
        println!("{}", asm);
        println!("-------------------------------\n");
    }

    if let Ok(v) = File::create("./tmp.s") {
        let mut file = v;
        match file.write_all(asm.as_bytes()) {
            Ok(()) => println!("success"),
            Err(_) => println!("failure")
        }
    }
}
