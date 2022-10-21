#[derive(Debug, PartialEq, Clone)]
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
    SEMI,
}
