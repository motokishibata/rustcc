use std::io::Write;
use std::fs::File;

use super::*;

pub fn compile(input: &str) {
    let mut input = input;

    // cargo runでは "-" 始まりの引数を渡せないためエスケープ用の文字を追加
    let first = &input[0..1];
    if first == "^" {
        input = &input[1..];
    }
    
    let tokens = token::tokenize(input);
    let top_node = parse::program(tokens);

    let mut asm_str = String::new();
    asm_str.push_str(".intel_syntax noprefix\n");
    asm_str.push_str(".globl main\n");
    asm_str.push_str("main:\n");

    // プロローグ
    // 変数26個分の領域を確保する
    asm_str.push_str("  push rbp\n");
    asm_str.push_str("  mov rbp, rsp\n");
    asm_str.push_str("  sub rsp, 208\n");

    //todo:複数回の呼び出しに対応？
    asm_str.push_str(&stackmachine::gen(top_node));
    asm_str.push_str("  pop rax\n");

    // エピローグ
    // 最後の式の結果がRAXに残っているのでそれが返り値になる
    
    asm_str.push_str("  mov rsp, rbp\n");
    asm_str.push_str("  pop rbp\n");
    asm_str.push_str("  ret\n");

    println!("{}", asm_str);

    if let Ok(v) = File::create("./tmp.s") {
        let mut file = v;
        match file.write_all(asm_str.as_bytes()) {
            Ok(()) => println!("success"),
            Err(_) => println!("failure")
        }
    }
}
