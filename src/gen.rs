use std::collections::HashMap;
use crate::parse::NodeType;

pub fn gen_x86(node: NodeType) -> String {
    let mut gen = Generator::new();
    gen.prologue();
    gen.gen_program(node);
    gen.epilogue();
    gen.src
}

struct Generator {
    src: String,
    labels: HashMap<String, i32>,
}

impl Generator {
    fn new() -> Self {
        let mut src = String::new();
        src.push_str(".intel_syntax noprefix\n");
        src.push_str(".globl main\n");
        src.push_str("main:\n");
        Generator {
            src,
            labels: HashMap::new()
        }
    }

    fn get_unique_label(&mut self, name: String) -> String {
        let count = match self.labels.get(&name) {
            Some(val) => val.clone(),
            None => {
                self.labels.insert(name.clone(), 1);
                1
            }
        };
        let fmt = format!("{}{}", name, count.to_string());
        self.labels.insert(name, count + 1);
        fmt
    }

    fn prologue(&mut self) {
        self.src.push_str("  push rbp\n");
        self.src.push_str("  mov rbp, rsp\n");
        self.src.push_str("  sub rsp, 208\n");
    }

    fn epilogue(&mut self) {
        self.src.push_str("  mov rsp, rbp\n");
        self.src.push_str("  pop rbp\n");
        self.src.push_str("  ret\n");
    }

    fn gen_program(&mut self, node: NodeType) {
        let stmts = match node {
            NodeType::Program(stmts) => stmts,
            _ => panic!("failure gen_program")
        };
        for stmt in stmts {
            self.gen_stmt(stmt);
            self.src.push_str("  pop rax\n");
        }
    }

    fn gen_stmt(&mut self, node: NodeType) {
        match node {
            NodeType::Return(expr) => {
                self.gen_expr(*expr);
                self.src.push_str("  pop rax\n");
                self.src.push_str("  mov rsp, rbp\n");
                self.src.push_str("  pop rbp\n");
                self.src.push_str("  ret\n");
            },
            NodeType::If(expr, then, els) => {
                self.gen_expr(*expr);
                self.src.push_str("  pop rax\n");
                self.src.push_str("  cmp rax, 0\n");
                let end_label = self.get_unique_label(".Lend".into());
                match els {
                    Some(stmt) => {
                        let else_label = self.get_unique_label(".Lelse".into());
                        self.src.push_str(&format!("je  {}\n", else_label));
                        self.gen_stmt(*then);
                        self.src.push_str(&format!("jmp {}\n", end_label));
                        self.src.push_str(&format!("{}:\n", else_label));
                        self.gen_stmt(*stmt);
                        self.src.push_str(&format!("{}:\n", end_label));
                    },
                    None => {
                        self.src.push_str(&format!("je {}\n", end_label));
                        self.gen_stmt(*then);
                        self.src.push_str(&format!("{}:\n", end_label));
                    }
                }
            }
            _ => self.gen_expr(node)
        }
    }

    fn gen_expr(&mut self, node: NodeType) {
        match node {
            NodeType::Num(val) => {
                self.src.push_str(&format!("  push {}\n", val));
                return;
            },
            NodeType::LVar(_) => {
                self.gen_lval(node);
                self.src.push_str("  pop rax\n");
                self.src.push_str("  mov rax, [rax]\n");
                self.src.push_str("  push rax\n");
                return;
            }
            NodeType::Assign(lhs, rhs) => {
                self.gen_lval(*lhs);
                self.gen_expr(*rhs);
                self.src.push_str("  pop rdi\n");
                self.src.push_str("  pop rax\n");
                self.src.push_str("  mov [rax], rdi\n");
                self.src.push_str("  push rdi\n");
                return;
            },
            NodeType::Negative(rhs) => {
                self.src.push_str("  push 0\n");
                self.gen_expr(*rhs);
                self.src.push_str("  pop rdi\n");
                self.src.push_str("  pop rax\n");
                self.src.push_str("  sub rax, rdi\n")
            },
            NodeType::Plus(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);
                self.src.push_str("  pop rdi\n");
                self.src.push_str("  pop rax\n");
                self.src.push_str("  add rax, rdi\n")
            },
            NodeType::Minus(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);
                self.src.push_str("  pop rdi\n");
                self.src.push_str("  pop rax\n");
                self.src.push_str("  sub rax, rdi\n")
            },
            NodeType::Mul(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);
                self.src.push_str("  pop rdi\n");
                self.src.push_str("  pop rax\n");
                self.src.push_str("  imul rax, rdi\n")
            },
            NodeType::Div(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);
                self.src.push_str("  pop rdi\n");
                self.src.push_str("  pop rax\n");
                self.src.push_str("  cqo\n");
                self.src.push_str("  idiv rdi\n");
            },
            NodeType::Eq(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);
                self.src.push_str("  pop rdi\n");
                self.src.push_str("  pop rax\n");
                self.src.push_str("  cmp rax, rdi\n");
                self.src.push_str("  sete al\n");
                self.src.push_str("  movzb rax, al\n");
            },
            NodeType::Ne(lhs, rhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);
                self.src.push_str("  pop rdi\n");
                self.src.push_str("  pop rax\n");
                self.src.push_str("  cmp rax, rdi\n");
                self.src.push_str("  setne al\n");
                self.src.push_str("  movzb rax, al\n");
            },
            NodeType::Lt(lhs, rhs) | NodeType::Gt(rhs, lhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);
                self.src.push_str("  pop rdi\n");
                self.src.push_str("  pop rax\n");
                self.src.push_str("  cmp rax, rdi\n");
                self.src.push_str("  setl al\n");
                self.src.push_str("  movzb rax, al\n");
            },
            NodeType::Le(lhs, rhs) | NodeType::Ge(rhs, lhs) => {
                self.gen_expr(*lhs);
                self.gen_expr(*rhs);
                self.src.push_str("  pop rdi\n");
                self.src.push_str("  pop rax\n");
                self.src.push_str("  cmp rax, rdi\n");
                self.src.push_str("  setle al\n");
                self.src.push_str("  movzb rax, al\n");
            },
            _ => {}
        }
        self.src.push_str("  push rax\n");
    }

    fn gen_lval(&mut self, node: NodeType) {
        let offset  = match node {
            NodeType::LVar(offset) => offset,
            _ => panic!("not lvar"),
        };
        self.src.push_str("  mov rax, rbp\n");
        self.src.push_str(format!("  sub rax, {}\n", offset).as_str());
        self.src.push_str("  push rax\n");
    }
}