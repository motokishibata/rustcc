use super::token;

pub fn to_asmstr(tokens: Vec<token::Token>) -> String {

    let mut s = String::from(".intel_syntax noprefix\n");
    s.push_str(".globl main\n");
    s.push_str("main:\n");

    let mut count = 0;
    let val = &tokens[count].val.unwrap().to_string();
    s.push_str(format!("  mov rax, {}\n", val).as_str());
    count += 1;

    while count < tokens.len() {
        let tok = &tokens[count];

        match tok.kind {
            token::TokenKind::Reserved => {
                let op = tok.ch.unwrap();
                count += 1;
                let val = &tokens[count].val.unwrap().to_string();
                if op == '+' {
                    s.push_str(format!("  add rax, {}\n", val).as_str());
                }
                else if op == '-' {
                    s.push_str(format!("  sub rax, {}\n", val).as_str());
                }
                count += 1;
            },
            token::TokenKind::Eof => break,
            _ => panic!("invalid token kind"),
        }
    }

    s.push_str("  ret\n");
    return s;
}