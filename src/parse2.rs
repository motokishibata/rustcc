use crate::TokenType;

pub fn parse(tokens: &Vec<TokenType>) -> NodeType {
    let locals: &mut Vec<LVar> = &mut vec![];
    let mut parser = Parser::new(tokens, locals);
    parser.program()
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Num(i32),       // 数値
    Plus(Box<NodeType>, Box<NodeType>),
    Minus(Box<NodeType>, Box<NodeType>),
    Mul(Box<NodeType>, Box<NodeType>),
    Div(Box<NodeType>, Box<NodeType>),
    Eq(Box<NodeType>, Box<NodeType>),
    Ne(Box<NodeType>, Box<NodeType>),
    Le(Box<NodeType>, Box<NodeType>),
    Lt(Box<NodeType>, Box<NodeType>),
    Ge(Box<NodeType>, Box<NodeType>),
    Gt(Box<NodeType>, Box<NodeType>),
    Negative(Box<NodeType>),        // -
    LVar(i32),
    Assign(Box<NodeType>, Box<NodeType>),
    Return(Box<NodeType>),
    Program(Vec<NodeType>),        // top node
}

#[derive(Debug, Clone)]
pub struct LVar {
    pub name: String,
    pub len: i32,
    pub offset: i32,
}

pub struct Parser<'a> {
    tokens: &'a Vec<TokenType>,
    pos: usize,
    locals: &'a mut Vec<LVar>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<TokenType>, locals: &'a mut Vec<LVar>) -> Self {
        Parser {
            tokens,
            pos: 0,
            locals,
        }
    }

    fn find_lvar(&self, ty: TokenType) -> Option<&LVar> {
        let ident = match ty {
            TokenType::Ident(name) => name,
            _ => return None,
        };
        for var in self.locals.iter() {
            if var.name == ident {
                return Some(var);
            }
        }
        return None;
    }

    fn expect(&mut self, ty: TokenType) {
        if let Some(t) = self.tokens.get(self.pos) {
            if *t == ty {
                self.pos += 1;
                return;
            }
        }
        panic!("not expect charcter");
    }

    fn consume(&mut self, ty: TokenType) -> bool {
        if let Some(t) = self.tokens.get(self.pos) {
            if *t == ty {
                self.pos += 1;
                return true;
            }
        }
        false
    }

    // program = stmt*
    fn program(&mut self) -> NodeType {
        let mut stmts = vec![];
        while self.tokens.len() != self.pos {
            stmts.push(self.stmt());
        }
        NodeType::Program(stmts)
    }

    // stmt =  expr ";" |
    //         "return" expr ";"
    fn stmt(&mut self) -> NodeType {
        let t = &self.tokens[self.pos];

        match t {
            TokenType::Return => {
                self.pos += 1;
                let expr = self.expr();
                self.expect(TokenType::Semicolon);
                NodeType::Return(Box::new(expr))
            },
            _ => {
                let expr = self.expr();
                self.expect(TokenType::Semicolon);
                expr
            }
        }
    }

    // expr = assign
    fn expr(&mut self) -> NodeType {
        self.assign()
    }

    // assign = equality ("=" assign)?
    fn assign(&mut self) -> NodeType {
        let eq = self.equality();
        if self.consume(TokenType::Assign) {
            let assign = Box::new(self.assign());
            NodeType::Assign(Box::new(eq), assign)
        } else {
            eq
        }
    }

    // equality = relational ("==" relational | "!=" relational)*
    fn equality(&mut self) -> NodeType {
        let rel = self.relational();
        if self.consume(TokenType::Eq) {
            let rhs = Box::new(self.relational());
            NodeType::Eq(Box::new(rel), rhs)
        } else if self.consume(TokenType::Ne) {
            let rhs = Box::new(self.relational());
            NodeType::Ne(Box::new(rel), rhs)
        } else {
            rel
        }
    }

    // relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    fn relational(&mut self) -> NodeType {
        let add = self.add();
        if self.consume(TokenType::Lt) {
            let rhs = Box::new(self.add());
            NodeType::Lt(Box::new(add), rhs)
        } else if self.consume(TokenType::Le) {
            let rhs = Box::new(self.add());
            NodeType::Le(Box::new(add), rhs)
        } else if self.consume(TokenType::Gt) {
            let rhs = Box::new(self.add());
            NodeType::Gt(Box::new(add), rhs)
        } else if self.consume(TokenType::Ge) {
            let rhs = Box::new(self.add());
            NodeType::Ge(Box::new(add), rhs)
        } else {
            add
        }
    }

    // add = mul ("+" mul | "-" mul)*
    fn add(&mut self) -> NodeType {
        let mul = self.mul();
        if self.consume(TokenType::Plus) {
            let rhs = Box::new(self.mul());
            NodeType::Plus(Box::new(mul), rhs)
        } else if self.consume(TokenType::Minus) {
            let rhs = Box::new(self.mul());
            NodeType::Minus(Box::new(mul), rhs)
        } else {
            mul
        }
    }

    // mul = unary ("*" unary | "/" unary)*
    fn mul(&mut self) -> NodeType {
        let unary = self.unary();
        if self.consume(TokenType::Mul) {
            let rhs = Box::new(self.unary());
            NodeType::Mul(Box::new(unary), rhs)
        } else if self.consume(TokenType::Div) {
            let rhs = Box::new(self.unary());
            NodeType::Div(Box::new(unary), rhs)
        } else {
            unary
        }
    }

    // unary = ("+" | "-")? primary
    fn unary(&mut self) -> NodeType {
        if self.consume(TokenType::Minus) {
            let primary = Box::new(self.primary());
            NodeType::Negative(primary)
        } else {
            self.primary()
        }
    }

    // primary = num | ident | "(" expr ")"
    fn primary(&mut self) -> NodeType {
        let t = &self.tokens[self.pos];
        match t {
            TokenType::Num(val) => {
                self.pos += 1;
                NodeType::Num(*val)
            },
            TokenType::Ident(ident) => {
                self.pos += 1;
                let offset;
                if let Some(lvar) = self.find_lvar(t.clone()) {
                    offset = lvar.offset;
                } else {
                    let tail = self.locals.last();
                    offset = match tail {
                        Some(var) => var.offset + 8,
                        None => 8,
                    };
                }
                let lvar = LVar {
                    name: ident.into(),
                    len: ident.len() as i32,
                    offset
                };
                self.locals.push(lvar);
                NodeType::LVar(offset)
            },
            _ => {
                self.expect(TokenType::LeftParen);
                let nt = self.expr();
                self.expect(TokenType::RightParen);
                nt
            }
        }
    }
}