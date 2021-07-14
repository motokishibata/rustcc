pub fn to_asmstr(input: &str) -> String {

    let mut s = String::from(".intel_syntax noprefix\n");
    s.push_str(".globl main\n");
    s.push_str("main:\n");

    let mut count = 0;
    let nlen = len_number(&input[count..]);
    s.push_str(format!("  mov rax, {}\n", &input[count..(count+nlen)]).as_str());
    count += nlen;

    while count < input.chars().count() {
        let ch = &input[count..(count+1)];
        if ch == "+" {
            count += 1;
            let nlen = len_number(&input[count..]);
            s.push_str(format!("  add rax, {}\n", &input[count..(count+nlen)]).as_str());
            count += nlen;
            continue;
        }
        if ch == "-" {
            count += 1;
            let nlen = len_number(&input[count..]);
            s.push_str(format!("  sub rax, {}\n", &input[count..(count+nlen)]).as_str());
            count += nlen;
            continue;
        }
        
        println!("想定外");
    }

    s.push_str("  ret\n");
    return s;
}

pub fn len_number(src: &str) -> usize {
    let mut counter = 0;
    for ch in src.chars() {
        let num: i32 = ch as i32 - 48;
        if 0 <= num && num <= 9 {
            counter += 1;
        }
        else {
            break;
        }
    }
    return counter;
}