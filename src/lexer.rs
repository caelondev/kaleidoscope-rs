use crate::tokens::Token;

pub struct Lexer {
    identifier_str: String,
    num_val: f64,
    last_char: char,

    src: Vec<char>,
    src_ptr: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        Self {
            identifier_str: String::new(),
            num_val: 0.0,
            last_char: ' ',
            src: src.chars().collect(),
            src_ptr: 0,
        }
    }

    pub fn get_tok(&mut self) -> Token {
        while self.last_char.is_whitespace() {
            self.last_char = self.get_char();
        }

        // Comment until end of line
        if self.last_char == '#' {
            loop {
                self.last_char = self.get_char();
                if self.is_at_end() || self.last_char == '\n' || self.last_char == '\r' {
                    break;
                }
            }

            if !self.is_at_end() {
                return self.get_tok();
            }
        }

        // EOF
        if self.is_at_end() {
            return Token::Eof;
        }

        // Get identifier / keyword
        if self.last_char.is_alphabetic() {
            self.identifier_str = self.last_char.to_string();

            loop {
                self.last_char = self.get_char();
                if !self.last_char.is_alphanumeric() {
                    break;
                }
                self.identifier_str.push(self.last_char);
            }

            if self.identifier_str_is("def") {
                return Token::Def;
            }

            if self.identifier_str_is("extern") {
                return Token::Extern;
            }

            return Token::Identifier;
        }

        // Get number
        if self.last_char.is_digit(10) || self.last_char == '.' {
            let mut num_str = String::new();

            loop {
                num_str.push(self.last_char);
                self.last_char = self.get_char();

                if !self.last_char.is_digit(10) && self.last_char != '.' {
                    break;
                }
            }

            self.num_val = num_str.parse().unwrap();
            return Token::Number;
        }

        // Unknown character
        let this_char = self.last_char;
        self.last_char = self.get_char();
        return Token::Unknown(this_char);
    }

    pub fn is_at_end(&self) -> bool {
        self.src_ptr >= self.src.len()
    }

    fn identifier_str_is(&self, s: &str) -> bool {
        self.identifier_str == s
    }

    fn get_char(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let c = self.src[self.src_ptr];
        self.src_ptr += 1;
        c
    }
}
