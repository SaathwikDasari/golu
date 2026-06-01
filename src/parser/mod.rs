use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::lexer::token::Token;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        // We are calling next_token function twice to so both the current and the peek can be populated.
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            current_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        /* Current token should become what the peek token was...
        We are using std::mem::replace to do this safely without fighting for the borrow checker... */

        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let name = match &self.peek_token {
            Token::Ident(ident_name) => ident_name.clone(),
            _ => return None,
        };

        self.next_token();

        if self.peek_token != Token::Assign {
            return None;
        }

        self.next_token();
        self.next_token();

        // TODO: I have to parse the mathematical expression here.... Will do it later!

        while self.current_token != Token::SemiColon && self.current_token != Token::EOF {
            self.next_token();
        }

        Some(Statement::Let {
            name,
            value: Expression::IntegralLiteral(0),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;
    use crate::ast::Statement;
    use crate::lexer::Lexer;

    #[test]
    fn test_let_statements() {
        let input = String::from("let x = 10; let y = 20");
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        // In the test input given earlier, we have given only 2 statements so we expect 2
        assert_eq!(program.statements.len(), 2);

        // After checking no. of statements now we check branches
        let stmt1 = &program.statements[0];
        if let Statement::Let { name, value: _ } = stmt1 {
            assert_eq!(name, "x");
        } else {
            panic!("Expected Let, got: {:?}", stmt1);
        }

        // Now let's verify the second branch too
        let stmt2 = &program.statements[1];
        if let Statement::Let { name, value: _ } = stmt2 {
            assert_eq!(name, "y");
        } else {
            panic!("Expected Let, got: {:?}", stmt2);
        }
    }
}
