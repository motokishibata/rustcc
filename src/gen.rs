use crate::parse::NodeType;

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
    let stmts = match node {
        NodeType::Program(stmts) => stmts,
        _ => panic!("failure gen_program")
    };
    for stmt in stmts {
        s.push_str(&gen_stmt(stmt));
        s.push_str("  pop rax\n");
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
        NodeType::Num(val) => {
            s.push_str(&format!("  push {}\n", val));
            return s;
        },
        NodeType::LVar(_) => {
            s.push_str(&gen_lval(node));
            s.push_str("  pop rax\n");
            s.push_str("  mov rax, [rax]\n");
            s.push_str("  push rax\n");
            return s;
        }
        NodeType::Assign(lhs, rhs) => {
            s.push_str(&gen_lval(*lhs));
            s.push_str(&gen_expr(*rhs));
            s.push_str("  pop rdi\n");
            s.push_str("  pop rax\n");
            s.push_str("  mov [rax], rdi\n");
            s.push_str("  push rdi\n");
            return s;
        },
        NodeType::Negative(rhs) => {
            s.push_str("  push 0\n");
            s.push_str(&gen_expr(*rhs));
            s.push_str("  pop rdi\n");
            s.push_str("  pop rax\n");
            s.push_str("  sub rax, rdi\n")
        },
        NodeType::Plus(lhs, rhs) => {
            s.push_str(&gen_expr(*lhs));
            s.push_str(&gen_expr(*rhs));
            s.push_str("  pop rdi\n");
            s.push_str("  pop rax\n");
            s.push_str("  add rax, rdi\n")
        },
        NodeType::Minus(lhs, rhs) => {
            s.push_str(&gen_expr(*lhs));
            s.push_str(&gen_expr(*rhs));
            s.push_str("  pop rdi\n");
            s.push_str("  pop rax\n");
            s.push_str("  sub rax, rdi\n")
        },
        NodeType::Mul(lhs, rhs) => {
            s.push_str(&gen_expr(*lhs));
            s.push_str(&gen_expr(*rhs));
            s.push_str("  pop rdi\n");
            s.push_str("  pop rax\n");
            s.push_str("  imul rax, rdi\n")
        },
        NodeType::Div(lhs, rhs) => {
            s.push_str(&gen_expr(*lhs));
            s.push_str(&gen_expr(*rhs));
            s.push_str("  pop rdi\n");
            s.push_str("  pop rax\n");
            s.push_str("  cqo\n");
            s.push_str("  idiv rdi\n");
        },
        NodeType::Eq(lhs, rhs) => {
            s.push_str(&gen_expr(*lhs));
            s.push_str(&gen_expr(*rhs));
            s.push_str("  pop rdi\n");
            s.push_str("  pop rax\n");
            s.push_str("  cmp rax, rdi\n");
            s.push_str("  sete al\n");
            s.push_str("  movzb rax, al\n");
        },
        NodeType::Ne(lhs, rhs) => {
            s.push_str(&gen_expr(*lhs));
            s.push_str(&gen_expr(*rhs));
            s.push_str("  pop rdi\n");
            s.push_str("  pop rax\n");
            s.push_str("  cmp rax, rdi\n");
            s.push_str("  setne al\n");
            s.push_str("  movzb rax, al\n");
        },
        NodeType::Lt(lhs, rhs) | NodeType::Gt(rhs, lhs) => {
            s.push_str(&gen_expr(*lhs));
            s.push_str(&gen_expr(*rhs));
            s.push_str("  pop rdi\n");
            s.push_str("  pop rax\n");
            s.push_str("  cmp rax, rdi\n");
            s.push_str("  setl al\n");
            s.push_str("  movzb rax, al\n");
        },
        NodeType::Le(lhs, rhs) | NodeType::Ge(rhs, lhs) => {
            s.push_str(&gen_expr(*lhs));
            s.push_str(&gen_expr(*rhs));
            s.push_str("  pop rdi\n");
            s.push_str("  pop rax\n");
            s.push_str("  cmp rax, rdi\n");
            s.push_str("  setle al\n");
            s.push_str("  movzb rax, al\n");
        },
        _ => {}
    }
    s.push_str("  push rax\n");
    s
}
