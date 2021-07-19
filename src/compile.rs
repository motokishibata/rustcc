use std::collections::VecDeque;
use std::io::Write;
use std::fs::File;

use super::*;

pub fn compile(input: &str) {
    let tokens = token::tokenize(input);
    let (top_node, _) = parse::expr(VecDeque::from(tokens));

    let mut asm_str = String::new();
    asm_str.push_str(".intel_syntax noprefix\n");
    asm_str.push_str(".globl main\n");
    asm_str.push_str("main:\n");

    asm_str.push_str(&stackmachine::gen(top_node));

    asm_str.push_str("  pop rax\n");
    asm_str.push_str("  ret\n");

    if let Ok(v) = File::create("./tmp.s") {
        let mut file = v;
        match file.write_all(asm_str.as_bytes()) {
            Ok(()) => println!("success"),
            Err(_) => println!("failure")
        }
    }
}
