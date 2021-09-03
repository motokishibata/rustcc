use crate::gen_ir::{Function, IROp, IR};

const REGS: [&str; 7] = ["r10", "r11", "rbx", "r12", "r13", "r14", "r15"];
const ARGREGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

struct Generator {
    src: String,
    label: usize,
}

impl Generator {
    fn new() -> Self {
        Self { src: String::new(), label:0 }
    }

    fn emit(&mut self, s: &str) {
        self.src.push_str(&format!("{}\n", s));
    }

    fn gen(&mut self, f: Function) {
        use self::IROp::*;
        let ret = format!(".Lend{}", self.label);
        self.label += 1;

        self.emit(".intel_syntax noprefix");
        self.emit(&format!(".global {}", f.name));
        self.emit(&format!("{}:", f.name));
        self.emit("  push rbp");
        self.emit("  mov rbp, rsp");

        for ir in f.code {
            let lhs = ir.lhs.unwrap();
            let rhs = ir.rhs.unwrap_or(0);
            match ir.op {
                Imm => self.emit(&format!("  mov {}, {}", REGS[lhs], rhs as i32)),
                Add => self.emit(&format!("  add {}, {}", REGS[lhs], REGS[rhs])),
                Sub => self.emit(&format!("  sub {}, {}", REGS[lhs], REGS[rhs])),
                Mul => {
                    self.emit(&format!("  mov rax, {}", REGS[rhs]));
                    self.emit(&format!("  mul {}", REGS[lhs]));
                    self.emit(&format!("  mov {}, rax", REGS[lhs]));
                },
                Div => {
                    self.emit(&format!("  mov rax, {}", REGS[lhs]));
                    self.emit("cqo");
                    self.emit(&format!("  div {}", REGS[rhs]));
                    self.emit(&format!("  mov {}, rax", REGS[lhs]));
                },
                Return => {
                    self.emit(&format!("  mov rax, {}", REGS[lhs]));
                    self.emit(&format!("  jmp {}", ret));
                },
                _ => {}
            }
        }

        self.emit(&format!("{}:", ret));
        self.emit("  mov rsp, rbp");
        self.emit("  pop rbp");
        self.emit("  ret")
    }
}

pub fn gen_x86(fns: Vec<Function>) -> String {
    let mut generator = Generator::new();
    for f in fns {
        generator.gen(f);
    }
    generator.src
}