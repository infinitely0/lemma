use crate::tokens::{Symbol, Token};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .unwrap_or(&Token::Symbol(Symbol::EOF, 0))
    }

    pub fn advance(&mut self) -> &Token {
        let token = self.tokens.get(self.current);
        if let Some(token) = token {
            self.current += 1;
            token
        } else {
            &Token::Symbol(Symbol::EOF, 0)
        }
    }

    pub fn has_more(&self) -> bool {
        !matches!(self.peek(), Token::Symbol(Symbol::EOF, _))
    }

    pub fn location(&self) -> usize {
        let current = self.tokens.get(self.current);
        if let Some(token) = current {
            match token {
                Token::Identifier(_, line) => *line,
                Token::Symbol(_, line) => *line,
                Token::Operator(_, line) => *line,
                Token::Type(_, line) => *line,
                Token::Value(_, line) => *line,
                Token::Conditional(_, line) => *line,
            }
        } else {
            0
        }
    }

    pub fn advance_while(&mut self, condition: fn(&Token) -> bool) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.has_more() && condition(self.peek()) {
            let t = self.advance();
            tokens.push(t.clone());
        }
        tokens
    }
}
