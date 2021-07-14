use std::env;
use std::io::Write;
use std::fs::File;

mod compile;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("invalid arguments");
        return;
    }

    let asmstr = compile::to_asmstr(args[1].as_str());
    println!("---------asm---------");
    println!("{}", asmstr);
    println!("---------asm---------");

    if let Ok(v) = File::create("./tmp.s") {
        let mut file = v;
        match file.write_all(asmstr.as_bytes()) {
            Ok(()) => println!("success"),
            Err(_) => println!("failure")
        }
    }
}
