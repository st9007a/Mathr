#[derive(Debug, PartialEq)]
pub enum Token {
    NUMBER(f64),
    ID(String),
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    COMMA,
    DOT,
    ASSIGN,
    PI,
    E,
    SEMI,
    EOF,
}
