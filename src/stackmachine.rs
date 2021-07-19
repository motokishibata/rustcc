use super::parse::*;

// 再帰も含め文字列を結合する必要あり。
pub fn gen(node: Node) -> String {
    let mut s = String::new();

    if node.kind == NodeKind::Num {
        s.push_str(format!("  push {}\n", node.val).as_str());
        return s;
    }

    let lhs = (*node.lhs).unwrap();
    let s = s + &gen(lhs);
    let rhs = (*node.rhs).unwrap();
    let mut s = s + &gen(rhs);

    s.push_str("  pop rdi\n");
    s.push_str("  pop rax\n");

    match node.kind {
        NodeKind::Add => s.push_str("  add rax, rdi\n"),
        NodeKind::Sub => s.push_str("  sub rax, rdi\n"),
        NodeKind::Mul => s.push_str("  imul rax, rdi\n"),
        NodeKind::Div => {
            s.push_str("  cqo\n");
            s.push_str("  idiv rdi\n");
        },
        _ => panic!("num is not support"),
    }

    s.push_str("  push rax\n");
    return s;
}