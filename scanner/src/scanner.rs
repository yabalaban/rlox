use crate::token::Token;
use crate::token::Type;

pub(crate) struct Scanner {
    pub(crate) source: Vec<u8>, 
    pub(crate) start: usize, 
    pub(crate) current: usize, 
    pub(crate) line: usize,
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token_ty = self.next_token_ty();
        let token = Some(self.make_token(token_ty));
        self.start = self.current;
        token
    }
}

impl Scanner {
    fn make_token(&self, ty: Type) -> Token {
        Token {
            ty: ty,
            pos: self.start..self.current,
            line: self.line,
        }
    }

    fn next_token_ty(&mut self) -> Type {
        if self.is_at_end() {
            return Type::Eof
        }
        self.skip_whitespace();
        match self.advance() {
            ch if ch.is_digit(10) => self.number(),
            ch if ch.is_alphabetic() => self.identifier(),
            '"' => self.string(),
            '(' => Type::LeftParen,
            ')' => Type::RightParen,
            '{' => Type::LeftBrace,
            '}' => Type::RightBrace,
            ';' => Type::Semicolon,
            ',' => Type::Comma,
            '.' => Type::Dot,
            '-' => Type::Minus,
            '+' => Type::Plus,
            '*' => Type::Star,
            '!' => if self.check('=') { Type::BangEqual } else { Type::Bang },
            '=' => if self.check('=') { Type::EqualEqual } else { Type::Equal },
            '<' => if self.check('=') { Type::LessEqual } else { Type::Greater },
            '>' => if self.check('=') { Type::GreaterEqual } else { Type::Less },
            '/' => {
                if self.peek_n(1) == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    };
                    self.next_token_ty()
                } else {
                    Type::Slash
                }
            },
            _ => self.next_token_ty(),
        }
    }
}

impl Scanner {
    fn number(&mut self) -> Type {
        while self.peek().is_digit(10) {
            self.advance();
        };
        if self.peek() == '.' && self.peek_n(1).is_digit(10) {
            self.advance();
        }
        while self.peek().is_digit(10) {
            self.advance();
        };
        Type::Number
    }

    fn string(&mut self) -> Type {
        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }
        if self.is_at_end() { panic!("Unterminated string") }
        self.advance();
        Type::String
    }

    fn identifier(&mut self) -> Type {
        while self.peek().is_digit(10) || self.peek().is_alphabetic() {
            self.advance();
        }
        match self.source[self.start] as char {
            'a' => self.check_keyword(1, 2, "nd", Type::And),
            'c' => self.check_keyword(1, 4, "lass", Type::Class),
            'e' => self.check_keyword(1, 3, "lse", Type::Else),
            'f' if self.current - self.start > 1 => {
                match self.peek_n_at(1, self.start) {
                    'a' => self.check_keyword(2, 3, "lse", Type::False),
                    'o' => self.check_keyword(2, 1, "r", Type::For),
                    'u' => self.check_keyword(2, 1, "n", Type::Fun),
                    _ => Type::Identifier,
                }
            },
            'i' => self.check_keyword(1, 1, "f", Type::If),
            'n' => self.check_keyword(1, 2, "il", Type::Nil),
            'o' => self.check_keyword(1, 1, "r", Type::Or),
            'p' => self.check_keyword(1, 3, "rint", Type::Print),
            'r' => self.check_keyword(1, 5, "eturn", Type::Return),
            's' => self.check_keyword(1, 4, "uper", Type::Super),
            't' if self.current - self.start > 1 => {
                match self.peek_n_at(1, self.start) {
                    'h' => self.check_keyword(2, 2, "is", Type::This),
                    'r' => self.check_keyword(2, 2, "ue", Type::True),
                    _ => Type::Identifier,
                }
            },
            'v' => self.check_keyword(1, 2, "var", Type::Var),
            'w' => self.check_keyword(1, 4, "hile", Type::While),
            _ => Type::Identifier,
        }
    }

    fn check_keyword(&self, start: usize, len: usize, rest: &str, ty: Type) -> Type {
        let same_length = self.current - self.start == start + len; 
        let match_substr = self.source[self.start + start..self.current] == rest.as_bytes().to_vec();
        if same_length && match_substr {
            ty
        } else {
            Type::Identifier
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => { 
                    self.start += 1;
                    self.advance(); 
                },
                _ => break
            }
        }
    }
}

impl Scanner {
    fn advance(&mut self) -> char {
        let ch = self.peek();
        self.current += 1;
        ch
    }

    fn check(&mut self, ch: char) -> bool {
        if self.is_at_end() { false }
        else if self.peek() != ch { false }
        else {
            self.current += 1;
            true
        }
    }

    fn is_at_end(&self) -> bool {
        self.is_at_end_n(0)
    }

    fn is_at_end_n(&self, n: usize) -> bool {
        self.current + n >= self.source.len()
    }

    fn peek(&self) -> char {
        self.peek_n(0)
    }

    fn peek_n(&self, n: usize) -> char {
        if self.is_at_end_n(n) { '\0' } 
        else { self.source[self.current + n] as char }
    }

    fn peek_n_at(&self, n: usize, start: usize) -> char {
        self.source[start + n] as char
    }
}
