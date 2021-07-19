use std::env;

mod compile;
mod token;
mod parse;
mod stackmachine;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("invalid arguments");
        return;
    }

    compile::compile(args[1].as_str());
}
