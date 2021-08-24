use crate::parse2::NodeType;
use crate::parse2::LVar;

pub fn gen_x86(node: NodeType) -> String {
    let mut asm = String::new();
    asm.push_str(".intel_syntax noprefix\n");
    asm.push_str(".globl main\n");
    asm.push_str("main:\n");

    // プロローグ
    // 変数26個分の領域を確保する
    asm.push_str("  push rbp\n");
    asm.push_str("  mov rbp, rsp\n");
    asm.push_str("  sub rsp, 208\n");

    // 構文木からコード生成
    asm.push_str(&gen_program(node));

    // エピローグ
    // 最後の式の結果がRAXに残っているのでそれが返り値になる
    asm.push_str("  mov rsp, rbp\n");
    asm.push_str("  pop rbp\n");
    asm.push_str("  ret\n");
    asm
}

fn gen_lval(node: NodeType) -> String {
    let offset  = match node {
        NodeType::LVar(offset) => offset,
        // NodeType::Equality(r1, _) => {
        //     match *r1 {
        //         NodeType::Relational(a1, _) => {
        //             match *a1 {
        //                 NodeType::Multi(u, _) => {
        //                     match *u {
        //                         NodeType::Unary(_, primary) => {
        //                             match *primary {
        //                                 NodeType::LVar(offset) => offset,
        //                                 _ => panic!("oh!"),
        //                             }
        //                         },
        //                         _ => panic!("not primary"),
        //                     }
        //                 },
        //                 _ => panic!("oh!"),
        //             }
        //         },
        //         _ => panic!("oh!"),
        //     }
        // },
        _ => panic!("not lvar"),
    };

    let mut s = String::new();
    s.push_str("  mov rax, rbp\n");
    s.push_str(format!("  sub rax, {}\n", offset).as_str());
    s.push_str("  push rax\n");
    return s;
}

fn gen_program(node: NodeType) -> String {
    let mut s = String::new();
    match node {
        NodeType::Program(stmts) => {
            for stmt in stmts {
                s.push_str(&gen_stmt(stmt));
                s.push_str("  pop rax\n");
            }
        },
        _ => {}
    }
    s
}

fn gen_stmt(node: NodeType) -> String {
    let mut s = String::new();
    match node {
        NodeType::Return(expr) => {
            s.push_str(&gen_expr(*expr));
            s.push_str("  pop rax\n");
            s.push_str("  mov rsp, rbp\n");
            s.push_str("  pop rbp\n");
            s.push_str("  ret\n");
        },
        _ => s.push_str(&gen_expr(node))
    }
    s
}

fn gen_expr(node: NodeType) -> String {
    let mut s = String::new();
    match node {
        NodeType::Expr(assign) => {
            s.push_str(&gen_assign(*assign));
        },
        _ => {}
    }
    s
}

fn gen_assign(node: NodeType) -> String {
    let mut s = String::new();
    match node {
        NodeType::Assign(n1, n2) => {
            s.push_str(&gen_equality(*n1));
            if let Some(v) = n2 {
                s.push_str(&gen_assign(*v));
                s.push_str("  pop rdi\n");
                s.push_str("  pop rax\n");
                s.push_str("  mov [rax], rdi\n");
                s.push_str("  push rdi\n");
            }
        },
        _ => {}
    }
    s
}

fn gen_equality(node: NodeType) -> String {
    let mut s = String::new();
    match node {
        NodeType::Equality(r1, r2) => {
            s.push_str(&gen_relational(*r1));
            for (op, rel) in r2 {
                s.push_str(&gen_relational(rel));
                s.push_str("  pop rdi\n");
                s.push_str("  pop rax\n");
                match op {
                    NodeType::Eq => {
                        s.push_str("  cmp rax, rdi\n");
                        s.push_str("  sete al\n");
                        s.push_str("  movzb rax, al\n");
                    },
                    NodeType::Ne => {
                        s.push_str("  cmp rax, rdi\n");
                        s.push_str("  setne al\n");
                        s.push_str("  movzb rax, al\n");
                    },
                    _ => {}
                }
                s.push_str("  push rax\n");
            }
        },
        _ => {}
    }
    s
}

fn gen_relational(node: NodeType) -> String {
    let mut s = String::new();
    match node {
        NodeType::Relational(add1, add2) => {
            s.push_str(&gen_add(*add1));
            for (op, add) in add2 {
                s.push_str(&gen_add(add));
                s.push_str("  pop rdi\n");
                s.push_str("  pop rax\n");
                match op {
                    NodeType::Lt => {
                        s.push_str("  cmp rax, rdi\n");
                        s.push_str("  setl al\n");
                        s.push_str("  movzb rax, al\n");
                    },
                    NodeType::Le => {
                        s.push_str("  cmp rax, rdi\n");
                        s.push_str("  setle al\n");
                        s.push_str("  movzb rax, al\n");
                    },
                    NodeType::Gt => {
                        s.push_str("  cmp rax, rdi\n");
                        s.push_str("  setg al\n");
                        s.push_str("  movzb rax, al\n");
                    },
                    NodeType::Ge => {
                        s.push_str("  cmp rax, rdi\n");
                        s.push_str("  setge al\n");
                        s.push_str("  movzb rax, al\n");
                    },
                    _ => {}
                }
                s.push_str("  push rax\n");
            }
        },
        _ => {}
    }
    s
}

fn gen_add(node: NodeType) -> String {
    let mut s = String::new();
    match node {
        NodeType::Add(mul1, mul2) => {
            s.push_str(&gen_mul(*mul1));
            for (op, mul) in mul2 {
                s.push_str(&gen_mul(mul));
                s.push_str("  pop rdi\n");
                s.push_str("  pop rax\n");
                match op {
                    NodeType::Plus => s.push_str("  add rax, rdi\n"),
                    NodeType::Minus => s.push_str("  sub rax, rdi\n"),
                    _ => {}
                }
                s.push_str("  push rax\n");
            }
        },
        _ => {}
    }
    s
}

fn gen_mul(node: NodeType) -> String {
    let mut s = String::new();
    match node {
        NodeType::Multi(u1, u2) => {
            s.push_str(&gen_unary(*u1));
            for (op, unary) in u2 {
                s.push_str(&gen_unary(unary));
                s.push_str("  pop rdi\n");
                s.push_str("  pop rax\n");
                match op {
                    NodeType::Mul => s.push_str("  imul rax, rdi\n"),
                    NodeType::Div => {
                        s.push_str("  cqo\n");
                        s.push_str("  idiv rdi\n");
                    },
                    _ => {}
                }
                s.push_str("  push rax\n");
            }
        },
        _ => {}
    }
    s
}

fn gen_unary(node: NodeType) -> String {
    let mut s = String::new();
    match node {
        NodeType::Unary(op, primary) => {
            s.push_str(&gen_primary(*primary));
            match *op {
                NodeType::Minus => {
                    s.push_str("  push 0\n");
                    s.push_str("  pop rax\n");
                    s.push_str("  pop rdi\n");
                    s.push_str("  sub rax, rdi\n");
                    s.push_str("  push rax\n");
                },
                _ => {}
            }
        },
        _ => {}
    }
    s
}

fn gen_primary(node: NodeType) -> String {
    let mut s = String::new();
    match node {
        NodeType::Num(val) => s.push_str(&format!("  push {}\n", val)),
        NodeType::LVar(_) => {
            s.push_str(&gen_lval(node));
            // s.push_str("  pop rax\n");
            // s.push_str("  mov rax, [rax]\n");
            // s.push_str("  push rax\n");
        },
        _ => s.push_str(&gen_expr(node)),
    }
    s
}