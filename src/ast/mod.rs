use crate::lexer::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Indentifier(String),  // ex: 'x' or 'y'
    IntegralLiteral(i64), // ex: 5 or 10

    Infix {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    // let <name> = <value>
    Let { name: String, value: Expression },

    // Sometimes when we calculate something without assigning it like: '5+5;'
    ExpressionStatement(Expression),
}

// A massive list of statements executed one after the other
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}
