extern crate rustcc;
use rustcc::*;

use std::env;
use std::io::Write;
use std::fs::File;

mod token;
mod parse;
mod gen;

use token::*;
use parse::*;
use gen::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("invalid arguments");
        return;
    }
    let mut input = args[1].as_str();
    // cargo runでは "-" 始まりの引数を渡せないためエスケープ用の文字を追加
    let first = &input[0..1];
    if first == "^" {
        input = &input[1..];
    }

    let tokens = tokenize(input);
    let nodes = parse(&tokens);
    let asm = gen_x86(nodes);

    println!("{}", asm);

    if let Ok(v) = File::create("./tmp.s") {
        let mut file = v;
        match file.write_all(asm.as_bytes()) {
            Ok(()) => println!("success"),
            Err(_) => println!("failure")
        }
    }
}
