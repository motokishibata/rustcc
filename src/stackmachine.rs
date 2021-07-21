use super::parse::*;

fn gen_lval(node: Node) -> String {
    if node.kind != NodeKind::LVar {
        panic!("node is not lvar");
    }

    let mut s = String::new();
    s.push_str("  mov rax, rbp\n");
    s.push_str(format!("  sub rax, {}\n", node.offset).as_str());
    s.push_str("  push rax\n");
    return s;
}

pub fn gen(node: Node) -> String {
    let mut s = String::new();
    if node.kind == NodeKind::Return {
        let lhs = (*node.lhs).unwrap();
        s.push_str(gen(lhs).as_str());
        s.push_str("  pop rax\n");
        s.push_str("  mov rsp, rbp\n");
        s.push_str("  pop rbp\n");
        s.push_str("  ret\n");
        return s;
    }

    match node.kind {
        NodeKind::Num => {
            s.push_str(format!("  push {}\n", node.val).as_str());
            return s;
        },
        NodeKind::LVar => {
            s.push_str(gen_lval(node).as_str());
            s.push_str("  pop rax\n");
            s.push_str("  mov rax, [rax]\n");
            s.push_str("  push rax\n");
            return s;
        },
        NodeKind::Assign => {
            let lhs = (*node.lhs).unwrap();
            s.push_str(gen_lval(lhs).as_str());
            let rhs = (*node.rhs).unwrap();
            s.push_str(gen(rhs).as_str());

            s.push_str("  pop rdi\n");
            s.push_str("  pop rax\n");
            s.push_str("  mov [rax], rdi\n");
            s.push_str("  push rdi\n");
            return s;
        },
        _ => {}
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
        NodeKind::Eq => {
            s.push_str("  cmp rax, rdi\n");
            s.push_str("  sete al\n");
            s.push_str("  movzb rax, al\n");
        },
        NodeKind::Ne => {
            s.push_str("  cmp rax, rdi\n");
            s.push_str("  setne al\n");
            s.push_str("  movzb rax, al\n");
        },
        NodeKind::Lt => {
            s.push_str("  cmp rax, rdi\n");
            s.push_str("  setl al\n");
            s.push_str("  movzb rax, al\n");
        },
        NodeKind::Le => {
            s.push_str("  cmp rax, rdi\n");
            s.push_str("  setle al\n");
            s.push_str("  movzb rax, al\n");
        }
        _ => panic!("num is not support"),
    }

    s.push_str("  push rax\n");
    return s;
}