use crate::parse::NodeType;

pub fn gen_ir(nodes: Vec<NodeType>) -> Vec<Function> {
    let mut funcs = vec![];
    for node in nodes {
        match node {
            NodeType::Func(name, args, body, stacksize) => {
                let mut generator = IrGenerator::new(vec![]);

                for (i, arg) in args.iter().enumerate() {
                    if let NodeType::LVar(offset) = arg {
                        generator.store_arg(Some((*offset) as usize), Some(i));
                    } else {
                        unreachable!();
                    }
                }

                let stmts;
                if let NodeType::CompStmt(_stmts) = *body {
                    stmts = _stmts;
                } else {
                    unreachable!();
                }
                for stmt in stmts {
                    generator.gen_stmt(stmt);
                }
                
                funcs.push(Function::new(name,  generator.code.clone(), stacksize));
            },
            _ => panic!("supported is function only!")
        }
    }
    funcs
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub code: Vec<IR>,
    pub stacksize: usize,
}

impl Function {
    fn new(name: String, code: Vec<IR>, stacksize: usize) -> Self {
        Function {
            name,
            code,
            stacksize,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum IROp {
    Add,
    Sub,
    Mul,
    Div,
    Imm,
    Return,
    Call(String, usize),
    Label,
    Bprel,
    Load(u8),
    Store(u8),
    StoreArg(u8),
}

#[derive(Clone, Debug)]
pub struct IR {
    pub op: IROp,
    pub lhs: Option<usize>,
    pub rhs: Option<usize>,
}

impl IR {
    fn new(op: IROp, lhs: Option<usize>, rhs: Option<usize>) -> Self {
        IR { op, lhs, rhs }
    }
}

struct IrGenerator {
    code: Vec<IR>,
    num_regs: usize,
}

impl IrGenerator {
    fn new(code: Vec<IR>) -> Self {
        Self { code, num_regs: 0 }
    }

    fn add(&mut self, op: IROp, lhs: Option<usize>, rhs: Option<usize>) {
        self.code.push(IR::new(op, lhs, rhs));
    }

    fn load(&mut self, dst: Option<usize>, src: Option<usize>) {
        self.add(IROp::Load(8), dst, src);
    }

    fn store(&mut self, dst: Option<usize>, src: Option<usize>) {
        self.add(IROp::Store(8), dst, src);
    }

    fn store_arg(&mut self, bpoff: Option<usize>, argreg: Option<usize>) {
        self.add(IROp::StoreArg(8), bpoff, argreg);
    }

    fn gen_lval(&mut self, node: Box<NodeType>) -> Option<usize> {
        match *node {
            NodeType::LVar(offset) => {
                let r = Some(self.num_regs);
                self.num_regs += 1;
                self.add(IROp::Bprel, r, Some(offset as usize));
                r
            },
            _ => unreachable!(),
        }
    }

    fn gen_binop(&mut self, op: IROp, lhs: Box<NodeType>, rhs: Box<NodeType>) -> Option<usize> {
        let r1 = self.gen_expr(*lhs);
        let r2 = self.gen_expr(*rhs);
        self.add(op, r1, r2);
        r1
    }

    fn gen_expr(&mut self, node: NodeType) -> Option<usize> {
        match node {
            NodeType::Num(val) => {
                let r = Some(self.num_regs);
                self.num_regs += 1;
                self.add(IROp::Imm, r, Some(val as usize));
                r
            },
            NodeType::LVar(_) => {
                let r = self.gen_lval(Box::new(node.clone()));
                self.load(r, r);
                r
            },
            NodeType::Plus(lhs, rhs) => self.gen_binop(IROp::Add, lhs, rhs),
            NodeType::Minus(lhs, rhs) => self.gen_binop(IROp::Sub, lhs, rhs),
            NodeType::Mul(lhs, rhs) => self.gen_binop(IROp::Mul, lhs, rhs),
            NodeType::Div(lhs, rhs) => self.gen_binop(IROp::Div, lhs, rhs),
            NodeType::Assign(lhs, rhs) => {
                let rhs = self.gen_expr(*rhs);
                let lhs = self.gen_lval(lhs);
                self.store(lhs, rhs);
                rhs
            },
            _ => panic!("unknown node in expr")
        }
    }

    fn gen_stmt(&mut self, node: NodeType) {
        match node {
            NodeType::Return(expr) => {
                let r = self.gen_expr(*expr);
                self.add(IROp::Return, r, None);
            },
            NodeType::ExprStmt(expr) => {
                self.gen_expr(*expr);
            }
            _ => panic!("unknown node in stmt：{:?}", node)
        }
    }
}
