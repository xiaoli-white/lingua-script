use crate::ast::*;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0, errors: Vec::new() }
    }

    fn error(&mut self, msg: String) {
        self.errors.push(msg);
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn peek_ahead(&self, n: usize) -> &Token {
        self.tokens.get(self.pos + n).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> &Token {
        let t = self.tokens.get(self.pos).unwrap_or(&Token::EOF);
        self.pos += 1;
        t
    }

    #[allow(dead_code)]
    fn expect(&mut self, tok: &Token) -> bool {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(tok) {
            self.advance();
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn expect_token(&mut self, expected: &Token) -> bool {
        if self.peek() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect_peek(&mut self, tok: &Token) -> bool {
        if self.peek() == tok {
            self.advance();
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn is_keyword(t: &Token) -> bool {
        matches!(t,
            Token::Is | Token::Be | Token::Becomes | Token::Let |
            Token::When | Token::Otherwise | Token::End |
            Token::Repeat | Token::Times | Token::For | Token::Each | Token::In | Token::While |
            Token::Start | Token::Here | Token::Stop | Token::Exit | Token::With |
            Token::Say | Token::Ask | Token::And | Token::Save | Token::To |
            Token::Read | Token::Write |
            Token::Define | Token::A | Token::It | Token::Has | Token::Which |
            Token::On | Token::Create | Token::Destroy | Token::Make | Token::Public |
            Token::Instantiate | Token::Fresh |
            Token::Note | Token::That | Token::Refer | Token::From | Token::Chapter |
            Token::Beware | Token::InCase | Token::Of | Token::Regardless |
            Token::Attempt | Token::If | Token::Fails |
            Token::Raise | Token::Return | Token::Run | Token::Execute |
            Token::Convert | Token::Type |
            Token::Added | Token::Subtracted | Token::Multiplied | Token::Divided |
            Token::Remainder | Token::Square | Token::Root | Token::The |
            Token::Sum | Token::Product |
            Token::Not | Token::Or |
            Token::Greater | Token::Less | Token::Equal |
            Token::Using | Token::UsingCall |
            Token::Empty | Token::Null |
            Token::List | Token::Containing | Token::Map |
            Token::Add | Token::Remove |
            Token::As |
            Token::Capitalize | Token::Extends | Token::Input |
            Token::Interface | Token::Can | Token::Implements |
            Token::Super)
    }

    #[allow(dead_code)]
    fn is_expr_start(t: &Token) -> bool {
        matches!(t,
            Token::Number(_) | Token::String(_) |
            Token::Identifier(_) |
            Token::LParen | Token::Minus |
            Token::Not | Token::Square | Token::Root | Token::The |
            Token::Sum | Token::Product | Token::Remainder |
            Token::Type | Token::Convert |
            Token::Null | Token::Empty |
            Token::Instantiate | Token::Fresh |
            Token::Capitalize | Token::Input)
    }

    pub fn parse_program(&mut self) -> Program {
        let mut stmts = Vec::new();
        let mut guard = 0;
        while self.peek() != &Token::EOF {
            let pos_before = self.pos;
            let stmt = self.parse_stmt();
            stmts.push(stmt);
            while self.expect_peek(&Token::Dot) {}
            if self.pos == pos_before {
                self.error(format!("unexpected token {:?} at position {}", self.peek(), self.pos));
                self.advance();
            }
            guard += 1;
            if guard > 1000 { 
                self.error("too many statements, possible infinite loop".to_string());
                break; 
            }
        }
        Program { stmts }
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            Token::Let => self.parse_let(),
            Token::When => self.parse_when(),
            Token::Repeat => self.parse_repeat(),
            Token::For => self.parse_foreach(),
            Token::While => self.parse_while(),
            Token::Start => self.parse_start(),
            Token::Stop => { self.advance(); Stmt::Stop }
            Token::Exit => self.parse_exit(),
            Token::Say => self.parse_say(),
            Token::Ask => self.parse_ask(),
            Token::Read => self.parse_read(),
            Token::Write => self.parse_write(),
            Token::Define => self.parse_define(),
            Token::Note => { self.advance(); while self.peek() != &Token::Dot && self.peek() != &Token::EOF { self.advance(); } Stmt::Expression(Expr::Null) }
            Token::Beware => self.parse_beware(),
            Token::Attempt => self.parse_attempt(),
            Token::Raise => { self.advance(); Stmt::Raise(self.parse_expr()) }
            Token::Return => { self.advance(); if self.peek() != &Token::Dot && self.peek() != &Token::End { Stmt::Return(Some(self.parse_expr())) } else { Stmt::Return(None) } }
            Token::Refer => self.parse_refer(),
            Token::Chapter => self.parse_chapter(),
            Token::To => self.parse_func_def(),
            Token::Run | Token::Execute => self.parse_func_call_stmt(),
            Token::If => self.parse_if_fails(),
            Token::Add => self.parse_add_remove(),
            Token::Remove => self.parse_add_remove(),
            Token::Convert => self.parse_convert(),
            Token::Make => self.parse_make_public(),
            _ => {
                if let Some(name) = Parser::peek_as_name(self.peek()) {
                    if self.peek_ahead(1) == &Token::Is {
                        return self.parse_is_def(name);
                    }
                    if self.peek_ahead(1) == &Token::Becomes {
                        return self.parse_becomes(name);
                    }
                }
                let expr = self.parse_expr();
                if let Expr::Identifier(_) = &expr {
                    if self.peek() == &Token::Dot && self.peek_ahead(1) != &Token::Dot {
                        self.advance();
                        return Stmt::FuncCall {
                            func: expr,
                            args: Vec::new(),
                            result_var: None,
                        };
                    }
                }
                Stmt::Expression(expr)
            }
        }
    }

    fn parse_is_def(&mut self, name: String) -> Stmt {
        self.advance();
        self.advance();
        let value = self.parse_expr();
        Stmt::VarDef { name, value }
    }

    fn parse_becomes(&mut self, name: String) -> Stmt {
        self.advance();
        self.advance();
        let value = self.parse_expr();
        Stmt::Assign { name, value }
    }

    fn parse_let(&mut self) -> Stmt {
        self.advance();
        let name = self.expect_identifier();
        self.advance();
        let value = self.parse_expr();
        Stmt::VarDecl { name, value }
    }

    fn parse_when(&mut self) -> Stmt {
        self.advance();
        let condition = self.parse_expr();
        self.expect_peek(&Token::Colon);
        let body = self.parse_block_until(&[Token::Otherwise, Token::End]);
        let mut otherwise = Vec::new();
        if self.peek() == &Token::Otherwise {
            self.advance();
            self.expect_peek(&Token::Colon);
            otherwise = self.parse_block_until(&[Token::End]);
        }
        if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
        Stmt::If { condition, body, otherwise }
    }

    fn parse_repeat(&mut self) -> Stmt {
        self.advance();
        if self.expect_peek(&Token::Times) {
            let times = self.parse_expr();
            self.expect_peek(&Token::Colon);
            let body = self.parse_block_until(&[Token::End]);
            if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
            Stmt::Repeat { times, body }
        } else {
            let times = self.parse_expr();
            self.expect_peek(&Token::Times);
            self.expect_peek(&Token::Colon);
            let body = self.parse_block_until(&[Token::End]);
            if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
            Stmt::Repeat { times, body }
        }
    }

    fn parse_foreach(&mut self) -> Stmt {
        self.advance();
        self.expect_peek(&Token::Each);
        let var = self.expect_identifier();
        self.expect_peek(&Token::In);
        let collection = self.parse_expr();
        self.expect_peek(&Token::Colon);
        let body = self.parse_block_until(&[Token::End]);
        if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
        Stmt::ForEach { var, collection, body }
    }

    fn parse_while(&mut self) -> Stmt {
        self.advance();
        let condition = self.parse_expr();
        self.expect_peek(&Token::Colon);
        let body = self.parse_block_until(&[Token::End]);
        if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
        Stmt::While { condition, body }
    }

    fn parse_start(&mut self) -> Stmt {
        self.advance();
        self.expect_peek(&Token::Here);
        self.expect_peek(&Token::Colon);
        let body = self.parse_block_until(&[Token::End]);
        if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
        Stmt::StartHere(body)
    }

    fn parse_exit(&mut self) -> Stmt {
        self.advance();
        if self.expect_peek(&Token::With) {
            let code = self.parse_expr();
            Stmt::Exit(Some(code))
        } else {
            Stmt::Exit(None)
        }
    }

    fn parse_say(&mut self) -> Stmt {
        self.advance();
        Stmt::Say(self.parse_expr())
    }

    fn parse_ask(&mut self) -> Stmt {
        self.advance();
        let prompt = self.parse_expr();
        self.expect_peek(&Token::And);
        self.expect_peek(&Token::Save);
        self.expect_peek(&Token::To);
        let var = self.expect_identifier();
        Stmt::Ask { prompt, var }
    }

    fn parse_read(&mut self) -> Stmt {
        self.advance();
        let filename = self.parse_expr();
        self.expect_peek(&Token::And);
        self.expect_peek(&Token::Save);
        self.expect_peek(&Token::To);
        let var = self.expect_identifier();
        Stmt::ReadFile { filename, var }
    }

    fn parse_write(&mut self) -> Stmt {
        self.advance();
        let content = self.parse_expr();
        self.expect_peek(&Token::To);
        let filename = self.parse_expr();
        Stmt::WriteFile { content, filename }
    }

    fn parse_define(&mut self) -> Stmt {
        self.advance();
        let _a = self.advance();
        if self.peek() == &Token::Interface {
            return self.parse_interface_def();
        }
        let name = self.expect_identifier();
        let mut parent = None;
        let mut implements = Vec::new();
        loop {
            if self.peek() == &Token::Extends {
                self.advance();
                parent = Some(match self.advance() {
                    Token::Identifier(n) => n.clone(),
                    _ => panic!("expected parent class name"),
                });
            } else if self.peek() == &Token::Implements {
                self.advance();
                loop {
                    match self.advance() {
                        Token::Identifier(n) => implements.push(n.clone()),
                        _ => panic!("expected interface name"),
                    }
                    if self.peek() != &Token::Comma { break; }
                    self.advance();
                }
            } else { break; }
        }
        self.expect_peek(&Token::Colon);

        let mut fields = Vec::new();
        let mut constructor = None;
        let mut destructor = None;
        let mut methods = Vec::new();
        let mut publics = Vec::new();

        loop {
            if self.peek() == &Token::End { break; }
            match self.peek() {
                Token::It => {
                    self.advance();
                    self.expect_peek(&Token::Has);
                    let name = match self.advance() {
                        Token::Identifier(n) => n.clone(),
                        _ => panic!("expected field name"),
                    };
                    self.expect_peek(&Token::Which);
                    self.expect_peek(&Token::Is);
                    let value = self.parse_expr();
                    fields.push((name, value));
                    while self.expect_peek(&Token::Dot) {}
                }
                Token::On => {
                    self.advance();
                    if self.expect_peek(&Token::Create) {
                        let mut params = Vec::new();
                        if self.expect_peek(&Token::With) {
                            params = self.parse_param_list();
                        }
                        self.expect_peek(&Token::Colon);
                        let body = self.parse_block_until(&[Token::End]);
                        if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
                        constructor = Some(ClassMethod { name: "create".into(), params, body });
                    } else if self.expect_peek(&Token::Destroy) {
                        self.expect_peek(&Token::Colon);
                        let body = self.parse_block_until(&[Token::End]);
                        if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
                        destructor = Some(ClassMethod { name: "destroy".into(), params: vec![], body });
                    } else {
                        panic!("expected create or destroy after on");
                    }
                    while self.expect_peek(&Token::Dot) {}
                }
                Token::To => {
                    self.advance();
                    let name = self.parse_name_token();
                    let mut params = Vec::new();
                    if self.expect_peek(&Token::With) {
                        params = self.parse_param_list();
                    }
                    self.expect_peek(&Token::Colon);
                    let body = self.parse_block_until(&[Token::End]);
                    if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
                    methods.push(ClassMethod { name, params, body });
                    while self.expect_peek(&Token::Dot) {}
                }
                Token::Make => {
                    self.advance();
                    let name = self.parse_name_token();
                    self.expect_peek(&Token::Public);
                    publics.push(name);
                    while self.expect_peek(&Token::Dot) {}
                }
                Token::When => {
                    self.advance();
                    let name = self.parse_multi_word_method_name();
                    let mut params = Vec::new();
                    if self.expect_peek(&Token::With) {
                        params = self.parse_param_list();
                    }
                    self.expect_peek(&Token::Colon);
                    let body = self.parse_block_until(&[Token::End]);
                    if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
                    methods.push(ClassMethod { name, params, body });
                    while self.expect_peek(&Token::Dot) {}
                }
                _ => panic!("unexpected token in class body: {:?}", self.peek()),
            }
        }
        if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
        Stmt::ClassDef { name, parent, implements, fields, constructor, destructor, methods, publics }
    }

    fn parse_interface_def(&mut self) -> Stmt {
        self.advance();
        let name = self.expect_identifier();
        let mut extends = Vec::new();
        if self.peek() == &Token::Extends {
            self.advance();
            loop {
                match self.advance() {
                    Token::Identifier(n) => extends.push(n.clone()),
                    _ => panic!("expected interface name"),
                }
                if self.peek() != &Token::Comma { break; }
                self.advance();
            }
        }
        self.expect_peek(&Token::Colon);
        let mut methods = Vec::new();
        loop {
            if self.peek() == &Token::End { break; }
            match self.peek() {
                Token::Can => {
                    self.advance();
                    let method_name = self.parse_name_token();
                    let mut params = Vec::new();
                    if self.expect_peek(&Token::With) {
                        params = self.parse_param_list();
                    }
                    methods.push(InterfaceMethod { name: method_name, params });
                    while self.expect_peek(&Token::Dot) {}
                }
                _ => panic!("unexpected token in interface body: {:?}", self.peek()),
            }
        }
        if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
        Stmt::InterfaceDef { name, extends, methods }
    }

    #[allow(dead_code)]
    fn parse_instantiate_stmt(&mut self) -> Stmt {
        self.advance();
        let var = if self.peek_ahead(0) == &Token::Identifier("".into()) && self.peek_ahead(1) == &Token::Be {
            let tok = self.advance().clone();
            let name = match tok {
                Token::Identifier(n) => n,
                _ => String::new(),
            };
            self.advance();
            Some(name)
        } else { None };
        self.expect_peek(&Token::Instantiate);
        let class_name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => panic!("expected class name"),
        };
        let mut args = Vec::new();
        if self.expect_peek(&Token::With) {
            loop {
                args.push(self.parse_expr());
                if self.peek() == &Token::Dot || self.peek() == &Token::Comma {
                    if self.peek() == &Token::Comma { self.advance(); }
                    break;
                }
                if !self.expect_peek(&Token::Comma) { break; }
            }
        }
        Stmt::Instantiate { class_name, args, var }
    }

    fn expect_identifier(&mut self) -> String {
        let tok = self.advance().clone();
        if let Token::Identifier(n) = &tok {
            n.clone()
        } else if let Some(name) = Parser::token_to_name(&tok) {
            name
        } else {
            panic!("expected identifier, got {:?}", tok)
        }
    }

    fn parse_name_token(&mut self) -> String {
        let tok = self.advance().clone();
        Self::token_to_name(&tok).unwrap_or_else(|| {
            panic!("expected a name, got {:?}", self.tokens.get(self.pos - 1))
        })
    }

    fn parse_multi_word_method_name(&mut self) -> String {
        let mut parts = Vec::new();
        loop {
            match self.peek() {
                Token::With | Token::Colon | Token::LParen | Token::Dot | Token::EOF => break,
                _ => {
                    let tok = self.advance();
                    if let Some(name) = Self::token_to_name(tok) {
                        parts.push(name);
                    } else {
                        panic!("unexpected token in method name: {:?}", self.tokens.get(self.pos - 1));
                    }
                }
            }
        }
        if parts.is_empty() {
            panic!("expected method name after when");
        }
        parts.join("_")
    }

    fn parse_func_def(&mut self) -> Stmt {
        self.advance();
        let name = self.parse_name_token();
        let mut params = Vec::new();
        if self.expect_peek(&Token::With) {
            params = self.parse_param_list();
        }
        self.expect_peek(&Token::Colon);
        let body = self.parse_block_until(&[Token::End]);
        if !self.expect_peek(&Token::End) {
            if self.peek() == &Token::EOF {
                self.error("incomplete function definition, expected 'end'".to_string());
            } else {
                self.error("expected 'end'".to_string());
            }
        }
        Stmt::FuncDef { name, params, body }
    }

    fn parse_func_call_stmt(&mut self) -> Stmt {
        self.advance();
        let func = self.parse_expr();
        let mut args = Vec::new();
        if self.expect_peek(&Token::With) {
            loop {
                args.push(self.parse_expr());
                if self.peek() != &Token::Comma { break; }
                self.advance();
            }
        }
        let mut result_var = None;
        if self.expect_peek(&Token::And) {
            self.expect_peek(&Token::Save);
            self.expect_peek(&Token::To);
            if let Token::Identifier(n) = self.advance() {
                result_var = Some(n.clone());
            }
        }
        Stmt::FuncCall { func, args, result_var }
    }

    fn parse_if_fails(&mut self) -> Stmt {
        self.advance();
        self.expect_peek(&Token::It);
        self.expect_peek(&Token::Fails);
        self.expect_peek(&Token::Colon);
        let body = self.parse_block_until(&[Token::End]);
        if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
        Stmt::Block(body)
    }

    fn parse_beware(&mut self) -> Stmt {
        self.advance();
        self.expect_peek(&Token::Colon);
        let body = self.parse_block_until(&[Token::In, Token::Regardless, Token::End]);
        let mut catch_type = None;
        let mut catch_body = Vec::new();
        if self.peek() == &Token::In {
            self.advance();
            self.expect_peek(&Token::InCase);
            self.expect_peek(&Token::Of);
            if let Token::Identifier(t) = self.advance() {
                catch_type = Some(t.clone());
            }
            self.expect_peek(&Token::Colon);
            catch_body = self.parse_block_until(&[Token::Regardless, Token::End]);
        }
        let mut finally_body = Vec::new();
        if self.peek() == &Token::Regardless {
            self.advance();
            self.expect_peek(&Token::Colon);
            finally_body = self.parse_block_until(&[Token::End]);
        }
        if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
        Stmt::Try { body, catch_type, catch_body, finally_body }
    }

    fn parse_attempt(&mut self) -> Stmt {
        self.advance();
        self.expect_peek(&Token::To);
        self.expect_peek(&Token::Colon);
        let body = self.parse_block_until(&[Token::If, Token::Regardless, Token::End]);
        let mut catch_body = Vec::new();
        if self.peek() == &Token::If {
            self.advance();
            self.expect_peek(&Token::It);
            self.expect_peek(&Token::Fails);
            self.expect_peek(&Token::Colon);
            catch_body = self.parse_block_until(&[Token::Regardless, Token::End]);
        }
        let mut finally_body = Vec::new();
        if self.peek() == &Token::Regardless {
            self.advance();
            self.expect_peek(&Token::Colon);
            finally_body = self.parse_block_until(&[Token::End]);
        }
        if !self.expect_peek(&Token::End) { panic!("expected 'end'"); }
        Stmt::Try { body, catch_type: None, catch_body, finally_body }
    }

    fn parse_refer(&mut self) -> Stmt {
        self.advance();
        self.expect_peek(&Token::To);
        let mut symbols = Vec::new();
        if matches!(self.peek(), Token::Identifier(_)) {
            if self.peek_ahead(1) == &Token::Comma {
                loop {
                    symbols.push(self.expect_identifier());
                    if self.peek() == &Token::Comma { self.advance(); }
                    else { break; }
                }
                self.expect_peek(&Token::From);
            } else if self.peek_ahead(1) == &Token::From {
                symbols.push(self.expect_identifier());
                self.advance();
            }
        }
        let mut path = vec![self.parse_name_token()];
        while self.peek() == &Token::Of {
            self.advance();
            path.push(self.parse_name_token());
        }
        let mut alias = None;
        if self.peek() == &Token::As {
            self.advance();
            alias = Some(self.expect_identifier());
        }
        Stmt::Refer { path, symbols, alias }
    }

    fn parse_chapter(&mut self) -> Stmt {
        self.advance();
        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => panic!("expected chapter name"),
        };
        let mut stmts = Vec::new();
        while self.peek() != &Token::EOF {
            if let Token::Chapter = self.peek() { break; }
            stmts.push(self.parse_stmt());
            while self.expect_peek(&Token::Dot) {}
        }
        Stmt::Chapter { name, stmts }
    }

    fn parse_add_remove(&mut self) -> Stmt {
        let is_add = self.peek() == &Token::Add;
        self.advance();
        let element = self.parse_expr();
        let keyword = if is_add { Token::To } else { Token::From };
        self.expect_peek(&keyword);
        let list = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => panic!("expected list name"),
        };
        if is_add {
            Stmt::AddToList { element, list }
        } else {
            Stmt::RemoveFromList { element, list }
        }
    }

    fn parse_convert(&mut self) -> Stmt {
        self.advance();
        let expr = self.parse_expr();
        self.expect_peek(&Token::To);
        let target_type = match self.advance() {
            Token::Identifier(t) => t.clone(),
            _ => panic!("expected type name"),
        };
        let mut var = String::new();
        if self.expect_peek(&Token::And) {
            self.expect_peek(&Token::Save);
            self.expect_peek(&Token::To);
            if let Token::Identifier(n) = self.advance() {
                var = n.clone();
            }
        }
        Stmt::Convert { expr, target_type, var }
    }

    fn parse_make_public(&mut self) -> Stmt {
        self.advance();
        let _name = self.expect_identifier();
        self.expect_peek(&Token::Public);
        Stmt::Expression(Expr::Null)
    }

    fn parse_list_literal(&mut self) -> Expr {
        self.advance();
        self.expect_peek(&Token::Containing);
        let mut items = Vec::new();
        loop {
            if matches!(self.peek(), Token::Dot | Token::EOF) { break; }
            items.push(self.parse_expr());
            if !self.expect_peek(&Token::Comma) { break; }
        }
        Expr::ListLiteral(items)
    }

    fn parse_map_literal(&mut self) -> Expr {
        self.advance();
        self.expect_peek(&Token::With);
        let mut pairs = Vec::new();
        loop {
            if matches!(self.peek(), Token::Dot | Token::EOF) { break; }
            let key = match self.peek() {
                Token::String(s) => { let v = s.clone(); self.advance(); v }
                Token::Identifier(s) => { let v = s.clone(); self.advance(); v }
                _ => break,
            };
            self.expect_peek(&Token::As);
            let value = self.parse_expr();
            pairs.push((key, value));
            if !self.expect_peek(&Token::Comma) { break; }
        }
        Expr::MapLiteral(pairs)
    }

    fn parse_using_object(&mut self) -> Expr {
        match self.peek() {
            Token::Identifier(n) => {
                let name = n.clone();
                self.advance();
                if self.peek() == &Token::Of {
                    let mut parts = vec![name];
                    while self.peek() == &Token::Of {
                        self.advance();
                        parts.push(self.parse_name_token());
                    }
                    Expr::ModulePath(parts)
                } else {
                    Expr::Identifier(name)
                }
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_param_list(&mut self) -> Vec<String> {
        let mut params = Vec::new();
        loop {
            match self.peek() {
                Token::Identifier(n) => {
                    params.push(n.clone());
                    self.advance();
                    if self.peek() == &Token::Comma {
                        self.advance();
                    } else { break; }
                }
                _ => break,
            }
        }
        params
    }

    fn parse_block_until(&mut self, terminators: &[Token]) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        loop {
            if self.peek() == &Token::EOF { break; }
            if terminators.iter().any(|t| self.peek() == t) { break; }
            let pos_before = self.pos;
            let stmt = self.parse_stmt();
            stmts.push(stmt);
            while self.expect_peek(&Token::Dot) {}
            if self.pos == pos_before {
                self.advance();
            }
        }
        stmts
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Expr {
        let mut left = self.parse_and();
        while self.peek() == &Token::Or {
            self.advance();
            let right = self.parse_and();
            left = Expr::BinaryOp { op: BinOp::Or, left: Box::new(left), right: Box::new(right) };
        }
        left
    }

    fn parse_and(&mut self) -> Expr {
        let mut left = self.parse_comparison();
        while self.peek() == &Token::And {
            if self.peek_ahead(1) == &Token::Save {
                break;
            }
            self.advance();
            let right = self.parse_comparison();
            left = Expr::BinaryOp { op: BinOp::And, left: Box::new(left), right: Box::new(right) };
        }
        left
    }

    fn parse_comparison(&mut self) -> Expr {
        let left = self.parse_addition();

        match self.peek() {
            Token::Is => {
                let pos = self.pos;
                self.advance();

                if self.peek() == &Token::Greater {
                    self.advance();
                    self.expect_peek(&Token::Than);
                    if self.peek() == &Token::Or
                        && self.peek_ahead(1) == &Token::Equal
                        && self.peek_ahead(2) == &Token::To
                    {
                        self.advance();
                        self.advance();
                        self.advance();
                        let right = self.parse_addition();
                        return Expr::BinaryOp { op: BinOp::Ge, left: Box::new(left), right: Box::new(right) };
                    }
                    let right = self.parse_addition();
                    return Expr::BinaryOp { op: BinOp::Gt, left: Box::new(left), right: Box::new(right) };
                }
                if self.peek() == &Token::Less {
                    self.advance();
                    self.expect_peek(&Token::Than);
                    if self.peek() == &Token::Or
                        && self.peek_ahead(1) == &Token::Equal
                        && self.peek_ahead(2) == &Token::To
                    {
                        self.advance();
                        self.advance();
                        self.advance();
                        let right = self.parse_addition();
                        return Expr::BinaryOp { op: BinOp::Le, left: Box::new(left), right: Box::new(right) };
                    }
                    let right = self.parse_addition();
                    return Expr::BinaryOp { op: BinOp::Lt, left: Box::new(left), right: Box::new(right) };
                }
                if self.peek() == &Token::Equal {
                    self.advance();
                    self.expect_peek(&Token::To);
                    let right = self.parse_addition();
                    return Expr::BinaryOp { op: BinOp::Eq, left: Box::new(left), right: Box::new(right) };
                }
                if self.peek() == &Token::Not {
                    self.advance();
                    self.expect_peek(&Token::Equal);
                    self.expect_peek(&Token::To);
                    let right = self.parse_addition();
                    return Expr::BinaryOp { op: BinOp::Ne, left: Box::new(left), right: Box::new(right) };
                }
                self.pos = pos;
                left
            }
            Token::Not => {
                self.advance();
                if self.peek() == &Token::Equal {
                    self.advance();
                    self.expect_peek(&Token::To);
                    let right = self.parse_addition();
                    return Expr::BinaryOp { op: BinOp::Ne, left: Box::new(left), right: Box::new(right) };
                }
                left
            }
            Token::Isnt => {
                self.advance();
                let right = self.parse_addition();
                return Expr::BinaryOp { op: BinOp::Ne, left: Box::new(left), right: Box::new(right) };
            }
            _ => left,
        }
    }

    fn parse_addition(&mut self) -> Expr {
        let mut left = self.parse_multiplication();

        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_multiplication();
                    left = Expr::BinaryOp { op: BinOp::Add, left: Box::new(left), right: Box::new(right) };
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_multiplication();
                    left = Expr::BinaryOp { op: BinOp::Sub, left: Box::new(left), right: Box::new(right) };
                }
                Token::Added => {
                    self.advance();
                    self.expect_peek(&Token::To);
                    let right = self.parse_multiplication();
                    left = Expr::BinaryOp { op: BinOp::Add, left: Box::new(left), right: Box::new(right) };
                }
                Token::Subtracted => {
                    self.advance();
                    if self.expect_peek(&Token::From) {
                        let right = self.parse_multiplication();
                        left = Expr::BinaryOp { op: BinOp::Sub, left: Box::new(right), right: Box::new(left) };
                    } else {
                        self.expect_peek(&Token::By);
                        let right = self.parse_multiplication();
                        left = Expr::BinaryOp { op: BinOp::Sub, left: Box::new(left), right: Box::new(right) };
                    }
                }
                _ => break,
            }
        }
        left
    }

    fn parse_multiplication(&mut self) -> Expr {
        let mut left = self.parse_unary();

        loop {
            match self.peek() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_unary();
                    left = Expr::BinaryOp { op: BinOp::Mul, left: Box::new(left), right: Box::new(right) };
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_unary();
                    left = Expr::BinaryOp { op: BinOp::Div, left: Box::new(left), right: Box::new(right) };
                }
                Token::Multiplied => {
                    self.advance();
                    self.expect_peek(&Token::By);
                    let right = self.parse_unary();
                    left = Expr::BinaryOp { op: BinOp::Mul, left: Box::new(left), right: Box::new(right) };
                }
                Token::Divided => {
                    self.advance();
                    self.expect_peek(&Token::By);
                    let right = self.parse_unary();
                    left = Expr::BinaryOp { op: BinOp::Div, left: Box::new(left), right: Box::new(right) };
                }
                _ => break,
            }
        }
        left
    }

    fn parse_unary(&mut self) -> Expr {
        match self.peek() {
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary();
                Expr::UnaryOp { op: UnaryOp::Neg, expr: Box::new(expr) }
            }
            Token::Not => {
                self.advance();
                let expr = self.parse_unary();
                Expr::UnaryOp { op: UnaryOp::Not, expr: Box::new(expr) }
            }
            Token::Remainder => {
                self.advance();
                self.expect_peek(&Token::Of);
                let left = self.parse_unary();
                self.expect_peek(&Token::Divided);
                self.expect_peek(&Token::By);
                let right = self.parse_unary();
                Expr::BinaryOp { op: BinOp::Mod, left: Box::new(left), right: Box::new(right) }
            }
            Token::Square => {
                self.advance();
                self.expect_peek(&Token::Of);
                let expr = self.parse_unary();
                Expr::BinaryOp { op: BinOp::Pow, left: Box::new(expr), right: Box::new(Expr::Number(2.0)) }
            }
            Token::Root | Token::The => {
                self.advance();
                let n = if let Token::Number(num) = self.peek() {
                    let val = *num;
                    self.advance();
                    val
                } else { 2.0 };
                self.expect_peek(&Token::Root);
                self.expect_peek(&Token::Of);
                let expr = self.parse_unary();
                Expr::BinaryOp { op: BinOp::Pow, left: Box::new(expr), right: Box::new(Expr::Number(1.0 / n)) }
            }
            Token::Sum => {
                self.advance();
                self.expect_peek(&Token::Of);
                self.parse_addition()
            }
            Token::Product => {
                self.advance();
                self.expect_peek(&Token::Of);
                self.parse_multiplication()
            }
            Token::Type => {
                self.advance();
                self.expect_peek(&Token::Of);
                let expr = self.parse_unary();
                Expr::TypeOf(Box::new(expr))
            }
            Token::Capitalize => {
                self.advance();
                let expr = self.parse_unary();
                Expr::Capitalize(Box::new(expr))
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Expr {
        match self.peek().clone() {
            Token::Number(n) => { self.advance(); Expr::Number(n) }
            Token::String(s) => { self.advance(); Expr::String(s) }
            Token::True => { self.advance(); Expr::Bool(true) }
            Token::False => { self.advance(); Expr::Bool(false) }
            Token::Null | Token::Empty => { self.advance(); Expr::Null }
            Token::A => {
                self.advance();
                match self.peek() {
                    Token::List => return self.parse_list_literal(),
                    Token::Map => return self.parse_map_literal(),
                    _ => Expr::Identifier("a".to_string()),
                }
            }
            Token::Input => {
                self.advance();
                Expr::Input
            }
            Token::Super => {
                self.advance();
                if self.peek() == &Token::Of {
                    self.advance();
                    let method = self.parse_name_token();
                    let mut args = Vec::new();
                    if self.expect_peek(&Token::With) {
                        loop {
                            args.push(self.parse_expr());
                            if self.peek() != &Token::Comma { break; }
                            self.advance();
                        }
                    }
                    return Expr::SuperCall { method, args };
                }
                let mut expr = Expr::Identifier("super".to_string());
                loop {
                    match self.peek() {
                        Token::LParen => {
                            self.advance();
                            let mut args = Vec::new();
                            if self.peek() != &Token::RParen {
                                loop {
                                    args.push(self.parse_expr());
                                    if self.peek() != &Token::Comma { break; }
                                    self.advance();
                                }
                            }
                            self.expect_peek(&Token::RParen);
                            expr = Expr::Call { callee: Box::new(expr), args };
                        }
                        Token::With => {
                            if let Expr::Identifier(_) = expr {
                                let mut args = Vec::new();
                                self.advance();
                                loop {
                                    args.push(self.parse_expr());
                                    if self.peek() != &Token::Comma { break; }
                                    self.advance();
                                }
                                expr = Expr::Call { callee: Box::new(expr), args };
                            } else { break; }
                        }
                        Token::Using | Token::UsingCall => {
                            self.advance();
                            let object = self.parse_using_object();
                            let mut args = Vec::new();
                            if self.peek() == &Token::With {
                                self.advance();
                                loop {
                                    args.push(self.parse_expr());
                                    if self.peek() != &Token::Comma { break; }
                                    self.advance();
                                }
                            }
                            if let Expr::Identifier(mname) = &expr {
                                expr = Expr::MethodCall {
                                    object: Box::new(object),
                                    method: mname.clone(),
                                    args,
                                };
                            }
                        }
                        Token::And => {
                            if self.peek_ahead(1) == &Token::Save {
                                break;
                            }
                            break;
                        }
                        _ => break,
                    }
                }
                expr
            }
            Token::Instantiate => {
                self.advance();
                let class_name = match self.advance() {
                    Token::Identifier(n) => n.clone(),
                    _ => panic!("expected class name after instantiate"),
                };
                let mut args = Vec::new();
                if self.expect_peek(&Token::With) {
                    loop {
                        args.push(self.parse_expr());
                        if !self.expect_peek(&Token::Comma) { break; }
                    }
                }
                Expr::Call {
                    callee: Box::new(Expr::Identifier(format!("__instantiate__{}", class_name))),
                    args,
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr();
                self.expect_peek(&Token::RParen);
                expr
            }
            Token::Identifier(name) => {
                self.advance();
                if name == "a" || name == "an" {
                    match self.peek() {
                        Token::List => return self.parse_list_literal(),
                        Token::Map => return self.parse_map_literal(),
                        _ => {}
                    }
                }
                let mut expr = Expr::Identifier(name.clone());

                loop {
                    match self.peek() {
                        Token::LParen => {
                            self.advance();
                            let mut args = Vec::new();
                            if self.peek() != &Token::RParen {
                                loop {
                                    args.push(self.parse_expr());
                                    if self.peek() != &Token::Comma { break; }
                                    self.advance();
                                }
                            }
                            self.expect_peek(&Token::RParen);
                            expr = Expr::Call { callee: Box::new(expr), args };
                        }
                        Token::With => {
                            if let Expr::Identifier(_) = expr {
                                let mut args = Vec::new();
                                self.advance();
                                loop {
                                    args.push(self.parse_expr());
                                    if self.peek() != &Token::Comma { break; }
                                    self.advance();
                                }
                                expr = Expr::Call { callee: Box::new(expr), args };
                            } else { break; }
                        }
                        Token::Using | Token::UsingCall => {
                            self.advance();
                            let object = self.parse_using_object();
                            let mut args = Vec::new();
                            if self.peek() == &Token::With {
                                self.advance();
                                loop {
                                    args.push(self.parse_expr());
                                    if self.peek() != &Token::Comma { break; }
                                    self.advance();
                                }
                            }
                            if let Expr::Identifier(mname) = &expr {
                                expr = Expr::MethodCall {
                                    object: Box::new(object),
                                    method: mname.clone(),
                                    args,
                                };
                            }
                        }
                        Token::And => {
                            if self.peek_ahead(1) == &Token::Save {
                                break;
                            }
                            break;
                        }
                        _ => break,
                    }
                }
                expr
            }
            Token::Fresh => {
                self.advance();
                let class_name = match self.advance() {
                    Token::Identifier(n) => n.clone(),
                    _ => panic!("expected class name"),
                };
                let mut args = Vec::new();
                if self.expect_peek(&Token::With) {
                    loop {
                        args.push(self.parse_expr());
                        if self.peek() != &Token::Comma { break; }
                        self.advance();
                    }
                }
                Expr::Call {
                    callee: Box::new(Expr::Identifier(format!("__instantiate__{}", class_name))),
                    args,
                }
            }
            _ => {
                let tok = self.advance();
                if let Some(name) = Parser::peek_as_name(tok) {
                    Expr::Identifier(name)
                } else {
                    Expr::Null
                }
            }
        }
    }

    fn peek_as_name(tok: &Token) -> Option<String> {
        if let Token::Identifier(n) = tok {
            Some(n.clone())
        } else {
            Parser::token_to_name(tok)
        }
    }

    fn token_to_name(tok: &Token) -> Option<String> {
        use Token::*;
        Some(match tok {
            Identifier(n) => n.clone(),
            Add => "add".into(),
            Remove => "remove".into(),
            Make => "make".into(),
            Say => "say".into(),
            Ask => "ask".into(),
            Read => "read".into(),
            Write => "write".into(),
            Stop => "stop".into(),
            Start => "start".into(),
            Run => "run".into(),
            Execute => "execute".into(),
            Return => "return".into(),
            Raise => "raise".into(),
            Convert => "convert".into(),
            Type => "type".into(),
            Is => "is".into(),
            Isnt => "isnt".into(),
            Be => "be".into(),
            Becomes => "becomes".into(),
            Let => "let".into(),
            Has => "has".into(),
            List => "list".into(),
            Map => "map".into(),
            Null => "null".into(),
            Empty => "empty".into(),
            True => "true".into(),
            False => "false".into(),
            Not => "not".into(),
            Or => "or".into(),
            And => "and".into(),
            As => "as".into(),
            In => "in".into(),
            Of => "of".into(),
            By => "by".into(),
            Than => "than".into(),
            To => "to".into(),
            Capitalize => "capitalize".into(),
            Extends => "extends".into(),
            Input => "input".into(),
            Interface => "interface".into(),
            Can => "can".into(),
            Implements => "implements".into(),
            Super => "super".into(),
            With => "with".into(),
            From => "from".into(),
            When => "when".into(),
            Otherwise => "otherwise".into(),
            End => "end".into(),
            Repeat => "repeat".into(),
            Times => "times".into(),
            For => "for".into(),
            Each => "each".into(),
            While => "while".into(),
            Here => "here".into(),
            Exit => "exit".into(),
            Save => "save".into(),
            Define => "define".into(),
            A => "a".into(),
            It => "it".into(),
            Which => "which".into(),
            On => "on".into(),
            Create => "create".into(),
            Destroy => "destroy".into(),
            Public => "public".into(),
            Instantiate => "instantiate".into(),
            Fresh => "fresh".into(),
            Note => "note".into(),
            That => "that".into(),
            Refer => "refer".into(),
            Chapter => "chapter".into(),
            Beware => "beware".into(),
            InCase => "incase".into(),
            Regardless => "regardless".into(),
            Attempt => "attempt".into(),
            If => "if".into(),
            Fails => "fails".into(),
            Greater => "greater".into(),
            Less => "less".into(),
            Equal => "equal".into(),
            Added => "added".into(),
            Subtracted => "subtracted".into(),
            Multiplied => "multiplied".into(),
            Divided => "divided".into(),
            Remainder => "remainder".into(),
            Square => "square".into(),
            Root => "root".into(),
            Sum => "sum".into(),
            Product => "product".into(),
            The => "the".into(),
            Using => "using".into(),
            UsingCall => "using".into(),
            _ => return None,
        })
    }
}
