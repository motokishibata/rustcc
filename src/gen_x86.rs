use crate::gen_ir::{Function, IROp, IR};

const REGS: [&str; 7] = ["r10", "r11", "rbx", "r12", "r13", "r14", "r15"];
const REGS8: [&str; 7] = ["r10b", "r11b", "bl", "r12b", "r13b", "r14b", "r15b"];
const ARGREGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

fn reg(r: usize, size: u8) -> &'static str {
    match size {
        1 => REGS8[r],
        8 => REGS[r],
        _ => unreachable!(),
    }
}

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

    fn emit_cmp(&mut self, ir:IR, s: &str) {
        let lhs = ir.lhs.unwrap();
        let rhs = ir.rhs.unwrap();
        self.emit(&format!("  cmp {}, {}", REGS[lhs], REGS[rhs]));
        self.emit(&format!("  {} {}", s, REGS8[lhs]));
        self.emit(&format!("  movzb {}, {}", REGS[lhs], REGS8[lhs]));
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
                Eq => self.emit_cmp(ir, "sete"),
                Ne => self.emit_cmp(ir, "setne"),
                Lt => self.emit_cmp(ir, "setl"),
                Le => self.emit_cmp(ir, "setle"),
                Label => self.emit(&format!(".L{}:", lhs)),
                Return => {
                    self.emit(&format!("  mov rax, {}", REGS[lhs]));
                    self.emit(&format!("  jmp {}", ret));
                },
                Jmp => self.emit(&format!("  jmp .L{}", lhs)),
                Unless => {
                    self.emit(&format!("  cmp {}, 0", REGS[lhs]));
                    self.emit(&format!("  je .L{}", rhs));
                },
                Load(size) => {
                    self.emit(&format!("  mov {}, [{}]", reg(lhs, size), REGS[rhs]));
                    if size == 1 {
                        self.emit(&format!("  movzb {}, {}", REGS[lhs], REGS8[lhs]));
                    }
                },
                Store(size) => self.emit(&format!("  mov [{}], {}", REGS[lhs], reg(rhs, size))),
                Bprel => self.emit(&format!("  lea {}, [rbp-{}]", REGS[lhs], rhs)),
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