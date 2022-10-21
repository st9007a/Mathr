use std::iter::Peekable;
use std::vec::IntoIter;

use crate::ast::{
    ASTSemanticExpression, AssignNode, BinaryOpNode, BinaryOpType, NumberNode, StatementListNode,
    UnaryOpNode, UnaryOpType, VarNode,
};
use crate::error::InterpreterError;
use crate::token::Token;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Box<StatementListNode>, InterpreterError> {
        self.statement_list()
    }

    pub fn variable(&mut self) -> Result<Box<VarNode>, InterpreterError> {
        if let Some(token) = self.next_token() {
            match token {
                Token::ID(value) => Ok(Box::new(VarNode::new(value.to_string()))),
                _ => Err(InterpreterError::EOF),
            }
        } else {
            Err(InterpreterError::EOF)
        }
    }

    pub fn assignment_statement(&mut self) -> Result<Box<AssignNode>, InterpreterError> {
        let var_node = self.variable()?;

        self.next_token()
            .ok_or(InterpreterError::EOF)
            .and_then(move |token| match token {
                Token::ASSIGN => Ok(Box::new(AssignNode::new(var_node, self.expr()?))),
                _ => Err(InterpreterError::UnexpectedToken(token)),
            })
    }

    pub fn statement(&mut self) -> Result<Box<AssignNode>, InterpreterError> {
        self.assignment_statement()
    }

    pub fn statement_list(&mut self) -> Result<Box<StatementListNode>, InterpreterError> {
        let mut nodes: Vec<Box<AssignNode>> = vec![self.statement()?];

        while let Some(token) = self.peek_token() {
            match token {
                Token::SEMI => {
                    self.next_token();
                    nodes.push(self.statement()?);
                }
                _ => {
                    break;
                }
            }
        }

        Ok(Box::new(StatementListNode::new(nodes)))
    }

    pub fn factor(&mut self) -> Result<Box<dyn ASTSemanticExpression>, InterpreterError> {
        if let Some(token) = self.peek_token() {
            match token {
                Token::PLUS => {
                    self.next_token();
                    Ok(Box::new(UnaryOpNode::new(
                        self.factor()?,
                        UnaryOpType::PLUS,
                    )))
                }
                Token::MINUS => {
                    self.next_token();
                    Ok(Box::new(UnaryOpNode::new(
                        self.factor()?,
                        UnaryOpType::MINUS,
                    )))
                }
                Token::NUMBER(value) => {
                    let node = Box::new(NumberNode::new(value.clone()));

                    self.next_token();
                    Ok(node)
                }
                Token::LPAREN => {
                    self.next_token();
                    let node = self.expr()?;

                    self.next_token()
                        .ok_or(InterpreterError::EOF)
                        .and_then(move |next_token| match next_token {
                            Token::RPAREN => Ok(node),
                            _ => Err(InterpreterError::UnexpectedToken(next_token)),
                        })
                }
                _ => self
                    .variable()
                    .map(|node| node as Box<dyn ASTSemanticExpression>),
            }
        } else {
            Err(InterpreterError::EOF)
        }
    }

    pub fn term(&mut self) -> Result<Box<dyn ASTSemanticExpression>, InterpreterError> {
        let mut left = self.factor()?;

        while let Some(token) = self.peek_token() {
            match token {
                Token::MUL => {
                    self.next_token();
                    left = Box::new(BinaryOpNode::new(left, self.factor()?, BinaryOpType::MUL));
                }
                Token::DIV => {
                    self.next_token();
                    left = Box::new(BinaryOpNode::new(left, self.factor()?, BinaryOpType::DIV));
                }
                _ => {
                    break;
                }
            }
        }

        Ok(left)
    }

    pub fn expr(&mut self) -> Result<Box<dyn ASTSemanticExpression>, InterpreterError> {
        let mut left = self.term()?;

        while let Some(token) = self.peek_token() {
            match token {
                Token::PLUS => {
                    self.next_token();
                    left = Box::new(BinaryOpNode::new(left, self.term()?, BinaryOpType::ADD));
                }
                Token::MINUS => {
                    self.next_token();
                    left = Box::new(BinaryOpNode::new(left, self.term()?, BinaryOpType::SUB));
                }
                _ => {
                    break;
                }
            }
        }

        Ok(left)
    }

    fn peek_token(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn next_token(&mut self) -> Option<Token> {
        self.tokens.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::ASTNode;
    use crate::symbol_table::SymbolTable;
    use crate::token::Token;

    use super::Parser;

    #[test]
    fn test_factor_integer() {
        let tokens = vec![Token::NUMBER(123.)];
        let mut parser = Parser::new(tokens);
        let mut symtab = SymbolTable::new();
        let expression = parser.factor();

        assert!(expression.is_ok());
        assert_eq!(expression.unwrap().eval(&mut symtab).unwrap(), 123f64);
    }

    #[test]
    fn test_term() {
        let tokens = vec![Token::NUMBER(4.), Token::MUL, Token::NUMBER(12.)];
        let mut parser = Parser::new(tokens);
        let mut symtab = SymbolTable::new();
        let expression = parser.term();

        assert!(expression.is_ok());
        assert_eq!(expression.unwrap().eval(&mut symtab).unwrap(), 48f64);
    }

    #[test]
    fn test_expr() {
        let tokens = vec![Token::NUMBER(4311.), Token::PLUS, Token::NUMBER(111.)];
        let mut parser = Parser::new(tokens);
        let mut symtab = SymbolTable::new();
        let expression = parser.expr();

        assert!(expression.is_ok());
        assert_eq!(expression.unwrap().eval(&mut symtab).unwrap(), 4422f64);
    }

    #[test]
    fn test_factor_parenthesis() {
        let tokens = vec![
            Token::LPAREN,
            Token::NUMBER(12.),
            Token::PLUS,
            Token::NUMBER(21.),
            Token::RPAREN,
        ];
        let mut parser = Parser::new(tokens);
        let mut symtab = SymbolTable::new();
        let expression = parser.factor();

        assert!(expression.is_ok());
        assert_eq!(expression.unwrap().eval(&mut symtab).unwrap(), 33f64);
    }

    #[test]
    fn test_factor_unary_op() {
        let tokens = vec![Token::PLUS, Token::PLUS, Token::NUMBER(12.)];
        let mut parser = Parser::new(tokens);
        let mut symtab = SymbolTable::new();
        let expression = parser.factor();

        assert!(expression.is_ok());
        assert_eq!(expression.unwrap().eval(&mut symtab).unwrap(), 12f64);
    }

    #[test]
    fn test_parse() {
        let tokens = vec![
            Token::ID("x".to_string()),
            Token::ASSIGN,
            Token::NUMBER(1.),
            Token::PLUS,
            Token::NUMBER(2.),
            Token::MUL,
            Token::LPAREN,
            Token::MINUS,
            Token::NUMBER(3.),
            Token::MINUS,
            Token::NUMBER(4.),
            Token::DIV,
            Token::NUMBER(2.),
            Token::RPAREN,
            Token::PLUS,
            Token::NUMBER(10.),
        ];
        let mut parser = Parser::new(tokens);
        let mut symtab = SymbolTable::new();
        let node = parser.parse();

        let x = "x".to_string();

        assert!(node.is_ok());
        assert_eq!(node.unwrap().eval(&mut symtab).unwrap(), 1f64);
        assert_eq!(symtab.get(&x), Some(&1f64));
    }
}
