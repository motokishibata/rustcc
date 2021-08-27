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
    Function(String, Vec<i32>),
    Assign(Box<NodeType>, Box<NodeType>),
    Return(Box<NodeType>),
    If(Box<NodeType>, Box<NodeType>, Option<Box<NodeType>>),
    While(Box<NodeType>, Box<NodeType>),
    For(Option<Box<NodeType>>, Option<Box<NodeType>>, Option<Box<NodeType>>, Box<NodeType>),
    Block(Vec<NodeType>),
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

    // stmt =  expr ";"
    //      | "{" stmt* "}"
    //      | "if" "(" expr ")" stmt ("else" stmt)?
    //      | "while" "(" expr ")" stmt
    //      |  "for" "(" expr? ";" expr? ";" expr? ")" stmt
    //      | "return" expr ";"
    fn stmt(&mut self) -> NodeType {
        let t = &self.tokens[self.pos];

        match t {
            TokenType::LeftBrace => {
                self.pos += 1;
                let mut stmts = vec![];
                while !self.consume(TokenType::RightBrace) {
                    stmts.push(self.stmt());
                }
                NodeType::Block(stmts)
            }
            TokenType::Return => {
                self.pos += 1;
                let expr = self.expr();
                self.expect(TokenType::Semicolon);
                NodeType::Return(Box::new(expr))
            },
            TokenType::If => {
                self.pos += 1;
                self.expect(TokenType::LeftParen);
                let expr = Box::new(self.expr());
                self.expect(TokenType::RightParen);
                let stmt = Box::new(self.stmt());
                let else_stmt;
                if self.consume(TokenType::Else) {
                    else_stmt = Some(Box::new(self.stmt()));
                } else {
                    else_stmt = None;
                }
                NodeType::If(expr, stmt, else_stmt)
            },
            TokenType::While => {
                self.pos += 1;
                self.expect(TokenType::LeftParen);
                let expr = Box::new(self.expr());
                self.expect(TokenType::RightParen);
                let stmt = Box::new(self.stmt());
                NodeType::While(expr, stmt)
            },
            TokenType::For => {
                self.pos += 1;
                self.expect(TokenType::LeftParen);
                let expr1;
                if self.consume(TokenType::Semicolon) {
                    expr1 = None;
                } else {
                    expr1 = Some(Box::new(self.expr()));
                    self.expect(TokenType::Semicolon);
                }
                let expr2;
                if self.consume(TokenType::Semicolon) {
                    expr2 = None;
                } else {
                    expr2 = Some(Box::new(self.expr()));
                    self.expect(TokenType::Semicolon);
                }
                let expr3;
                if self.consume(TokenType::RightParen) {
                    expr3 = None;
                } else {
                    expr3 = Some(Box::new(self.expr()));
                    self.expect(TokenType::RightParen);
                }
                let stmt = Box::new(self.stmt());
                NodeType::For(expr1, expr2, expr3, stmt)
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
        let mut rel = self.relational();
        loop {
            if self.consume(TokenType::Eq) {
                let rhs = Box::new(self.relational());
                rel = NodeType::Eq(Box::new(rel), rhs);
                continue;
            } else if self.consume(TokenType::Ne) {
                let rhs = Box::new(self.relational());
                rel = NodeType::Ne(Box::new(rel), rhs);
                continue;
            }
            break;
        }
        rel
    }

    // relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    fn relational(&mut self) -> NodeType {
        let mut add = self.add();
        loop {
            if self.consume(TokenType::Lt) {
                let rhs = Box::new(self.add());
                add = NodeType::Lt(Box::new(add), rhs);
                continue;
            } else if self.consume(TokenType::Le) {
                let rhs = Box::new(self.add());
                add = NodeType::Le(Box::new(add), rhs);
                continue;
            } else if self.consume(TokenType::Gt) {
                let rhs = Box::new(self.add());
                add = NodeType::Gt(Box::new(add), rhs);
                continue;
            } else if self.consume(TokenType::Ge) {
                let rhs = Box::new(self.add());
                add = NodeType::Ge(Box::new(add), rhs);
                continue;
            }
            break;
        }
        add
    }

    // add = mul ("+" mul | "-" mul)*
    fn add(&mut self) -> NodeType {
        let mut mul = self.mul();
        loop {
            if self.consume(TokenType::Plus) {
                let rhs = Box::new(self.mul());
                mul = NodeType::Plus(Box::new(mul), rhs);
                continue;
            } else if self.consume(TokenType::Minus) {
                let rhs = Box::new(self.mul());
                mul = NodeType::Minus(Box::new(mul), rhs);
                continue;
            }
            break;
        }
        mul
    }

    // mul = unary ("*" unary | "/" unary)*
    fn mul(&mut self) -> NodeType {
        let mut unary = self.unary();
        loop {
            if self.consume(TokenType::Mul) {
                let rhs = Box::new(self.unary());
                unary = NodeType::Mul(Box::new(unary), rhs);
                continue;
            } else if self.consume(TokenType::Div) {
                let rhs = Box::new(self.unary());
                unary = NodeType::Div(Box::new(unary), rhs);
                continue;
            }
            break;
        }
        unary
    }

    // unary = ("+" | "-")? primary
    fn unary(&mut self) -> NodeType {
        if self.consume(TokenType::Minus) {
            let primary = Box::new(self.primary());
            NodeType::Negative(primary)
        } else if self.consume(TokenType::Plus) {
            self.primary()
        } else {
            self.primary()
        }
    }

    // primary = num | ident ("(" ")")? | "(" expr ")"
    fn primary(&mut self) -> NodeType {
        let t = &self.tokens[self.pos];
        match t {
            TokenType::Num(val) => {
                self.pos += 1;
                NodeType::Num(*val)
            },
            TokenType::Ident(ident) => {
                self.pos += 1;
                if self.consume(TokenType::LeftParen) {
                    let mut args = vec![];
                    while !self.consume(TokenType::RightParen) {
                        let t = &self.tokens[self.pos];
                        match t {
                            TokenType::Num(val) => args.push(*val),
                            TokenType::Comma => {},
                            _ => panic!("function args error"),
                        }
                        self.pos += 1;
                    }
                    NodeType::Function(ident.clone(), args)
                } else {
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
                }
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