use crate::TokenType;

pub fn parse(tokens: &Vec<TokenType>) -> Vec<NodeType> {
    let locals: &mut Vec<LVar> = &mut vec![];
    let mut parser = Parser::new(tokens, locals);

    let mut nodes = vec![];
    while tokens.len() != parser.pos {
        nodes.push(parser.toplevel());
    }
    nodes
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
    Call(String, Vec<NodeType>),
    Func(String, Vec<NodeType>, Box<NodeType>, usize),  // ident, args, body, stacksize
    Assign(Box<NodeType>, Box<NodeType>),
    Return(Box<NodeType>),
    If(Box<NodeType>, Box<NodeType>, Option<Box<NodeType>>),
    While(Box<NodeType>, Box<NodeType>),
    For(Option<Box<NodeType>>, Option<Box<NodeType>>, Option<Box<NodeType>>, Box<NodeType>),
    Block(Vec<NodeType>),
    ExprStmt(Box<NodeType>),
    CompStmt(Vec<NodeType>),
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

    // トップレベルは関数である前提
    // TODO: グローバル変数とかは無視してる
    fn toplevel(&mut self) -> NodeType {
        let t = &self.tokens[self.pos];
        let name: String;
        if let TokenType::Ident(ident) = t {
            name = ident.clone();
        } else {
            panic!("not started ident is function")
        }
        self.pos += 1;

        // TODO: 一旦引数無しで定義できるように
        self.expect(TokenType::LeftParen);
        let args = vec![];
        self.expect(TokenType::RightParen);

        self.expect(TokenType::LeftBrace);
        let body = Box::new(self.compound_stmt());

        // TODO: 引数無しなのでスタックサイズも0
        NodeType::Func(name, args, body, 0)
    }

    // coumpound_stmt = stmt*
    fn compound_stmt(&mut self) -> NodeType {
        let mut stmts = vec![];
        while !self.consume(TokenType::RightBrace) {
            stmts.push(self.stmt());
        }
        NodeType::CompStmt(stmts)
    }

    // stmt =  expr_stmt ";"
    //      | "{" stmt* "}"
    //      | "if" "(" expr ")" stmt ("else" stmt)?
    //      | "while" "(" expr ")" stmt
    //      | "for" "(" expr? ";" expr? ";" expr? ")" stmt
    //      | "return" expr ";"
    fn stmt(&mut self) -> NodeType {
        let t = &self.tokens[self.pos];
        self.pos += 1;
        match t {
            TokenType::If => {
                self.expect(TokenType::LeftParen);
                let expr = Box::new(self.expr());
                self.expect(TokenType::RightParen);
                let stmt = Box::new(self.stmt());
                let mut else_stmt = None;
                if self.consume(TokenType::Else) {
                    else_stmt = Some(Box::new(self.stmt()));
                }
                NodeType::If(expr, stmt, else_stmt)
            },
            TokenType::For => {
                self.expect(TokenType::LeftParen);

                let mut init = None;
                if !self.consume(TokenType::Semicolon) {
                    init = Some(Box::new(self.expr()));
                    self.expect(TokenType::Semicolon);
                }

                let mut cond = None;
                if !self.consume(TokenType::Semicolon) {
                    cond = Some(Box::new(self.expr()));
                    self.expect(TokenType::Semicolon);
                }

                let mut inc = None;
                if !self.consume(TokenType::RightParen) {
                    inc = Some(Box::new(self.expr()));
                    self.expect(TokenType::RightParen);
                }

                let stmt = Box::new(self.stmt());
                NodeType::For(init, cond, inc, stmt)
            },
            TokenType::While => {
                self.expect(TokenType::LeftParen);
                let expr = Box::new(self.expr());
                self.expect(TokenType::RightParen);
                let stmt = Box::new(self.stmt());
                // TODO: 参考ソースでは、条件のみ指定したfor文としてノードを作っている
                NodeType::While(expr, stmt)
            },
            TokenType::Return => {
                let expr = self.expr();
                self.expect(TokenType::Semicolon);
                NodeType::Return(Box::new(expr))
            },
            TokenType::LeftBrace => {
                let mut stmts = vec![];
                while !self.consume(TokenType::RightBrace) {
                    stmts.push(self.stmt());
                }
                NodeType::Block(stmts)
            }
            _ => {
                // 代入式などがあるので必要
                self.pos -= 1;
                self.expr_stmt()
            }
        }
    }

    // expr_stmt = expr ";"
    fn expr_stmt(&mut self) -> NodeType {
        let expr = self.expr();
        self.expect(TokenType::Semicolon);
        NodeType::ExprStmt(Box::new(expr))
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
        self.pos += 1;
        match t {
            TokenType::Num(val) => NodeType::Num(*val),
            TokenType::Ident(ident) => {
                // 括弧が続かない場合はローカル変数
                if !self.consume(TokenType::LeftParen) {
                    let offset;
                    if let Some(lvar) = self.find_lvar(t.clone()) {
                        offset = lvar.offset;
                    } else {
                        let tail = self.locals.last();
                        offset = match tail {
                            Some(var) => var.offset + 4,
                            None => 4,
                        };
                    }
                    let lvar = LVar {
                        name: ident.into(),
                        len: ident.len() as i32,
                        offset
                    };
                    self.locals.push(lvar);
                    return NodeType::LVar(offset);
                }

                // 括弧が続くなら関数呼び出し
                let mut args = vec![];
                if self.consume(TokenType::RightParen) {
                    return NodeType::Call(ident.clone(), args);
                }

                // TODO: 参考ソースではassignだが、一旦引数での直接代入はサポートしない
                args.push(self.primary());
                while self.consume(TokenType::Comma) {
                    args.push(self.primary());
                }
                self.expect(TokenType::RightParen);
                NodeType::Call(ident.clone(), args)
            },
            TokenType::LeftParen => {
                let nt = self.expr();
                self.expect(TokenType::RightParen);
                nt
            },
            _ => panic!("failed primary")
        }
    }
}