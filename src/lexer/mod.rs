mod token;
use token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => Token::Assign,
            ';' => Token::SemiColon,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '\0' => Token::EOF,

            'a'..='z' | 'A'..='Z' | '_' => {
                let ident = self.read_identifier();

                return match ident.as_str() {
                    "let" => Token::Let,
                    _ => Token::Ident(ident),
                };
            }

            '0'..='9' => {
                return Token::Int(self.read_number());
            }
            _ => Token::Illegal,
        };

        self.read_char();
        tok
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        // Keep reading as long as it's a letter or an underscore
        while self.ch.is_ascii_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        // Slice the array from the start to the end of the word and turn it into a String
        self.input[position..self.position].iter().collect()
    }

    fn read_number(&mut self) -> i64 {
        let position = self.position;
        // Keep reading as long as it's a number (0-9)
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        let num_str: String = self.input[position..self.position].iter().collect();
        // For a toy compiler, unwrap() is fine here because we already guaranteed
        // the string only contains digits.
        num_str.parse::<i64>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::token::Token;
    use super::*;

    #[test]
    fn test_next_token() {
        let input = String::from("let x = 10; let y = x + 5;");

        let expected_tokens = vec![
            Token::Let,
            Token::Ident(String::from("x")),
            Token::Assign,
            Token::Int(10),
            Token::SemiColon,
            Token::Let,
            Token::Ident(String::from("y")),
            Token::Assign,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Int(5),
            Token::SemiColon,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        for (i, expected) in expected_tokens.into_iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(
                tok, expected,
                "Test failed at token {}: expected {:?}, got {:?}",
                i, expected, tok
            );
        }
    }
}
