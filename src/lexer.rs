use crate::{
    errors::CompilerError,
    scanner::Scanner,
    tokens::{
        Conditional::{Else, If, Then},
        Operator::{Add, And, Div, Eq, Gt, Gte, Lt, Lte, Mul, Neq, Not, Or, Sub},
        Symbol::{Assign, Bar, Pipe, Range, Return, EOL, LB, LP, RB, RP},
        Token,
        Type::{Bool, Frac, Int, Str, Void},
        Value::{self, Fractional, Integer},
    },
};

pub fn tokens(source: &str) -> Result<Vec<Token>, CompilerError> {
    let scanner = Scanner::new(source.to_string());
    let mut lexer = Lexer::new(scanner);
    lexer.tokens()
}

struct Lexer {
    scanner: Scanner,
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(scanner: Scanner) -> Self {
        Self {
            scanner,
            tokens: Vec::new(),
        }
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn line(&self) -> usize {
        self.scanner.line()
    }

    fn tokens(&mut self) -> Result<Vec<Token>, CompilerError> {
        while let Some(char) = self.scanner.advance() {
            match char {
                // Comments
                '#' => {
                    while let Some(char) = self.scanner.advance() {
                        if char == '\n' {
                            break;
                        }
                    }
                }
                // Newline - semicolon can be used as a line terminator
                '\n' | ';' => self.push(Token::Symbol(EOL, self.scanner.line())),
                // CR and CRLF line endings
                '\r' => {
                    if let Some(next) = self.scanner.peek() {
                        if next == '\n' {
                            self.scanner.advance();
                        }
                    }
                    self.push(Token::Symbol(EOL, self.scanner.line()));
                }
                // Whitespace
                ' ' | '\t' => {}
                // Single-character tokens
                '(' => self.push(Token::Symbol(LP, self.line())),
                ')' => self.push(Token::Symbol(RP, self.line())),
                '[' => self.push(Token::Symbol(LB, self.line())),
                ']' => self.push(Token::Symbol(RB, self.line())),
                '+' => self.push(Token::Operator(Add, self.line())),
                '*' => self.push(Token::Operator(Mul, self.line())),
                '/' => self.push(Token::Operator(Div, self.line())),
                // Double-character tokens
                '>' => match self.scanner.peek() {
                    Some('>') => {
                        self.push(Token::Symbol(Pipe, self.line()));
                        self.scanner.advance();
                    }
                    Some('=') => {
                        self.push(Token::Operator(Gte, self.line()));
                        self.scanner.advance();
                    }
                    _ => {
                        self.push(Token::Operator(Gt, self.line()));
                    }
                },
                '<' => {
                    if let Some('=') = self.scanner.peek() {
                        self.push(Token::Operator(Lte, self.line()));
                        self.scanner.advance();
                    } else {
                        self.push(Token::Operator(Lt, self.line()));
                    }
                }
                '-' => {
                    if let Some('>') = self.scanner.peek() {
                        self.push(Token::Symbol(Return, self.line()));
                        self.scanner.advance();
                    } else {
                        self.push(Token::Operator(Sub, self.line()));
                    }
                }
                '=' => {
                    if let Some('=') = self.scanner.peek() {
                        self.push(Token::Operator(Eq, self.line()));
                        self.scanner.advance();
                    } else {
                        self.push(Token::Symbol(Assign, self.line()));
                    }
                }
                '&' => {
                    if let Some('&') = self.scanner.peek() {
                        self.push(Token::Operator(And, self.line()));
                        self.scanner.advance();
                    } else {
                        return Err(CompilerError::Lexer(
                            "invalid token `&`".to_string(),
                            self.scanner.line(),
                        ));
                    }
                }
                '|' => {
                    if let Some('|') = self.scanner.peek() {
                        self.push(Token::Operator(Or, self.line()));
                        self.scanner.advance();
                    } else {
                        self.push(Token::Symbol(Bar, self.line()));
                    }
                }
                '!' => {
                    if let Some('=') = self.scanner.peek() {
                        self.push(Token::Operator(Neq, self.line()));
                        self.scanner.advance();
                    } else {
                        self.push(Token::Operator(Not, self.line()));
                    }
                }
                '.' => {
                    if let Some('.') = self.scanner.peek() {
                        self.push(Token::Symbol(Range, self.line()));
                        self.scanner.advance();
                    } else {
                        return Err(CompilerError::Lexer(
                            "invalid token `.`".to_string(),
                            self.scanner.line(),
                        ));
                    }
                }
                // Multi-character tokens
                c => {
                    if char.is_ascii_digit() {
                        self.tokenize_numeric()?
                    } else if c == '\"' {
                        self.tokenize_string()?
                    } else if char.is_ascii_alphabetic() {
                        self.tokenize_other()?
                    } else {
                        return Err(CompilerError::Lexer(
                            format!("invalid character `{}`", char),
                            self.scanner.line(),
                        ));
                    }
                }
            }
            self.scanner.reset_start();
        }
        Ok(self.tokens.clone())
    }

    fn tokenize_numeric(&mut self) -> Result<(), CompilerError> {
        while self.scanner.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.scanner.advance();
        }

        // Check for and handle fractional part
        if self.scanner.peek() == Some('.') {
            self.scanner.advance(); // Consume '.'

            if self.scanner.peek().map_or(false, |c| c.is_ascii_digit()) {
                while self.scanner.peek().map_or(false, |c| c.is_ascii_digit()) {
                    self.scanner.advance();
                }

                let lexeme = self.scanner.take_lexeme();
                return match lexeme.parse::<f64>() {
                    Ok(frac) => {
                        self.push(Token::Value(Fractional(frac), self.line()));
                        Ok(())
                    }
                    Err(_) => Err(CompilerError::Lexer(
                        format!("could not parse numeric `{}`", lexeme),
                        self.scanner.line(),
                    )),
                };
            }
        }

        // Handle integer part
        let lexeme = self.scanner.take_lexeme();
        match lexeme.parse::<i64>() {
            Ok(int) => {
                self.push(Token::Value(Integer(int), self.line()));
                Ok(())
            }
            Err(_) => Err(CompilerError::Lexer(
                format!("could not parse numeric `{}`", lexeme),
                self.scanner.line(),
            )),
        }
    }

    fn tokenize_string(&mut self) -> Result<(), CompilerError> {
        while self.scanner.peek().map_or(false, |c| c != '\"') {
            self.scanner.advance();
        }

        if self.scanner.peek() != Some('\"') {
            return Err(CompilerError::Lexer(
                "unterminated string".to_string(),
                self.scanner.line(),
            ));
        } else {
            self.scanner.advance(); // Include the closing `"`
        }

        let lexeme = self.scanner.take_lexeme();
        let stripped_lexeme = strip_string(&lexeme);
        self.scanner.advance(); // Consume closing `"`
        self.push(Token::Value(
            Value::String(stripped_lexeme.to_string()),
            self.line(),
        ));
        Ok(())
    }

    fn tokenize_other(&mut self) -> Result<(), CompilerError> {
        // Valid characters for identifiers other than a-z, A-Z, 0-9
        let valid_chars: Vec<char> = vec!['-']; // Kebab-case for variable names
        while let Some(char) = self.scanner.peek() {
            if char.is_ascii_alphanumeric() || valid_chars.contains(&char) {
                self.scanner.advance();
            } else {
                break;
            }
        }
        let lexeme = self.scanner.take_lexeme();

        // Booleans are the only literals not captured by the caller
        if let Ok(bool) = lexeme.parse::<bool>() {
            self.push(Token::Value(Value::Boolean(bool), self.line()));
            return Ok(());
        }

        // Keywords
        let token = match lexeme.as_str() {
            "if" => Some(Token::Conditional(If, self.line())),
            "then" => Some(Token::Conditional(Then, self.line())),
            "else" => Some(Token::Conditional(Else, self.line())),
            _ => None,
        };
        if let Some(token) = token {
            self.push(token);
            return Ok(());
        }

        // Types
        let token = match lexeme.as_str() {
            // Types
            "Int" => Some(Token::Type(Int, self.line())),
            "Frac" => Some(Token::Type(Frac, self.line())),
            "Str" => Some(Token::Type(Str, self.line())),
            "Bool" => Some(Token::Type(Bool, self.line())),
            "Void" => Some(Token::Type(Void, self.line())),
            _ => None,
        };
        if let Some(token) = token {
            self.push(token);
            return Ok(());
        }

        // All other tokens are assumed to be variable names
        self.push(Token::Identifier(lexeme, self.line()));
        Ok(())
    }
}

fn strip_string(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
