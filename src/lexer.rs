use crate::number;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Is, Isnt, Be, Becomes, Let, True, False,
    When, Otherwise, End,
    Repeat, Times, For, Each, In, While,
    Start, Here, Stop, Exit, With,
    Say, Ask, And, Save, To,
    Read, Write,
    Define, A, It, Has, Which, On, Create, Destroy, Make, Public,
    Instantiate, Fresh,
    Note, That,
    Refer, From, Chapter,
    Beware, InCase, Of, Regardless,
    Attempt, If, Fails,
    Raise, Return,
    Run, Execute,
    Convert, Type,
    Added, Subtracted, Multiplied, Divided,
    Remainder, Square, Root,
    Sum, Product, As,
    Not, Or,
    Greater, Less, Equal,
    Using, UsingCall,
    Empty, Null,
    List, Containing, Map,
    Add, Remove,
    The, By, Than,
    Number(f64),
    String(String),
    Identifier(String),
    Colon, Dot, Comma, LParen, RParen,
    Plus, Minus, Star, Slash,
    Illegal(String),
    EOF,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer { chars: input.chars().collect(), pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_ahead(&self, n: usize) -> Option<char> {
        self.chars.get(self.pos + n).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        self.pos += 1;
        c
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' || c == '\r' || c == '\n' {
                self.advance();
            } else { break; }
        }
    }

    fn skip_line_comment(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' { break; }
            self.advance();
        }
    }

    fn read_string(&mut self) -> String {
        let mut s = String::new();
        loop {
            match self.advance() {
                Some('"') => break,
                Some('\\') => {
                    match self.advance() {
                        Some('n') => s.push('\n'),
                        Some('t') => s.push('\t'),
                        Some('"') => s.push('"'),
                        Some('\\') => s.push('\\'),
                        Some(c) => s.push(c),
                        None => break,
                    }
                }
                Some(c) => s.push(c),
                None => break,
            }
        }
        s
    }

    fn read_multi_string(&mut self) -> String {
        let mut s = String::new();
        loop {
            if self.peek() == Some('"') && self.peek_ahead(1) == Some('"') && self.peek_ahead(2) == Some('"') {
                self.advance(); self.advance(); self.advance();
                break;
            }
            match self.advance() {
                Some(c) => s.push(c),
                None => break,
            }
        }
        s
    }

    fn read_number(&mut self, first: char) -> f64 {
        let mut s = String::new();
        s.push(first);
        let mut has_dot = false;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.advance();
            } else if c == '.' && !has_dot {
                if self.peek_ahead(1).map_or(false, |n| n.is_ascii_digit()) {
                    has_dot = true;
                    s.push(c);
                    self.advance();
                } else { break; }
            } else { break; }
        }
        s.parse::<f64>().unwrap_or(0.0)
    }

    fn read_ident(&mut self, first: char) -> String {
        let mut s = String::new();
        s.push(first);
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.advance();
            } else { break; }
        }
        s
    }

    fn try_read_number_words(&mut self, first_word: &str) -> Option<f64> {
        let mut words = vec![first_word.to_string()];
        loop {
            self.skip_ws();
            let start = self.pos;
            let mut w = String::new();
            while let Some(c) = self.peek() {
                if c.is_alphanumeric() || c == '_' {
                    w.push(c);
                    self.advance();
                } else { break; }
            }
            if w.is_empty() || (!number::is_number_word(&w) && w.to_lowercase() != "point") {
                self.pos = start;
                break;
            }
            words.push(w);
        }
        number::parse_english_number(&words)
    }

    fn kw(ident: &str) -> Option<Token> {
        use Token::*;
        Some(match ident {
            "is" => Is, "isnt" => Isnt, "be" => Be, "becomes" => Becomes, "let" => Let,
            "when" => When, "otherwise" => Otherwise, "end" => End,
            "repeat" => Repeat, "times" => Times, "for" => For,
            "each" => Each, "in" => In, "while" => While,
            "start" => Start, "here" => Here, "stop" => Stop,
            "exit" => Exit, "with" => With,
            "say" => Say, "ask" => Ask, "and" => And, "save" => Save,
            "to" => To, "read" => Read, "write" => Write,
            "true" => True, "false" => False,
            "define" => Define, "an" => A,
            "it" => It, "has" => Has, "which" => Which,
            "on" => On, "create" => Create, "destroy" => Destroy,
            "make" => Make, "public" => Public,
            "instantiate" => Instantiate, "fresh" => Fresh,
            "note" => Note, "that" => That,
            "refer" => Refer, "from" => From, "chapter" => Chapter,
            "beware" => Beware, "case" => InCase, "of" => Of,
            "regardless" => Regardless,
            "attempt" => Attempt, "if" => If, "fails" => Fails,
            "raise" => Raise, "return" => Return,
            "run" => Run, "execute" => Execute,
            "convert" => Convert, "type" => Type,
            "added" => Added, "subtracted" => Subtracted,
            "multiplied" => Multiplied, "divided" => Divided,
            "remainder" => Remainder, "square" => Square,
            "root" => Root, "sum" => Sum, "product" => Product,
            "not" => Not, "or" => Or, "as" => As,
            "greater" => Greater, "less" => Less, "equal" => Equal,
            "using" => Using,
            "empty" => Empty, "null" => Null,
            "list" => List, "containing" => Containing, "map" => Map,
            "add" => Add, "remove" => Remove, "the" => The,
            "by" => By, "than" => Than,
            _ => return None,
        })
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_ws();
        let c = match self.advance() {
            None => return Token::EOF,
            Some(c) => c,
        };

        if c == '"' {
            if self.peek() == Some('"') && self.peek_ahead(1) == Some('"') {
                self.advance(); self.advance();
                return Token::String(self.read_multi_string());
            }
            return Token::String(self.read_string());
        }

        if c.is_ascii_digit() {
            let n = self.read_number(c);
            self.skip_ws();
            let saved = self.pos;
            let mut w = String::new();
            while let Some(p) = self.peek() {
                if p.is_alphanumeric() || p == '_' {
                    w.push(p);
                    self.advance();
                } else { break; }
            }
            if !w.is_empty() && number::is_number_word(&w) {
                let s = format!("{} {}", n, w);
                let mut words = vec![s];
                loop {
                    self.skip_ws();
                    let saved2 = self.pos;
                    let mut w2 = String::new();
                    while let Some(p) = self.peek() {
                        if p.is_alphanumeric() || p == '_' {
                            w2.push(p);
                            self.advance();
                        } else { break; }
                    }
                    if w2.is_empty() || (!number::is_number_word(&w2) && w2.to_lowercase() != "point") {
                        self.pos = saved2;
                        break;
                    }
                    words.push(w2);
                }
                let parsed: Vec<String> = words.iter().flat_map(|s| s.split_whitespace().map(String::from)).collect();
                if let Some(val) = number::parse_english_number(&parsed) {
                    return Token::Number(val);
                }
            }
            if !w.is_empty() { self.pos = saved; }
            return Token::Number(n);
        }

        if c.is_alphabetic() || c == '_' {
            let ident = self.read_ident(c);
            if number::is_number_word(&ident) {
                if let Some(n) = self.try_read_number_words(&ident) {
                    return Token::Number(n);
                }
            }
            if let Some(kw) = Self::kw(&ident) {
                return kw;
            }
            return Token::Identifier(ident);
        }

        match c {
            ':' => Token::Colon,
            '.' => Token::Dot,
            ',' => Token::Comma,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => {
                if self.peek() == Some('/') {
                    self.advance();
                    self.skip_line_comment();
                    return self.next_token();
                }
                Token::Slash
            }
            _ => Token::Illegal(c.to_string()),
        }
    }
}
