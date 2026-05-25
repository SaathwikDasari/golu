#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,

    Ident(String), // variable names like x or y
    Int(i64),

    Assign, // =
    Plus,
    Minus,
    Asterisk,
    Slash,

    Semicolon,

    EOF,
    Illegal,
}
