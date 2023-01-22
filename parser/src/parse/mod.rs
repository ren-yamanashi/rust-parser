pub mod ast;
pub mod lex;
pub mod token;

// MEMO: まだちゃんと理解できてない
use std::io::{Error, ErrorKind};

use self::{
    ast::{Node, OperatorNode},
    lex::Lexer,
    token::Token,
};

pub fn parse(input: String) -> Result<Node, Error> {
    let mut p = Parser::new(input);
    p.parse()
}

struct Parser {
    lx: Lexer,
    current: Token,
    peeked: Token,
}

impl Parser {
    fn new(input: String) -> Self {
        let mut lx = Lexer::new(input);
        let current = lx.next();
        let peeked = lx.next();
        Parser {
            lx,
            current,
            peeked,
        }
    }

    fn parse(&mut self) -> Result<Node, Error> {
        self.parse_by_current_precedence(0)
    }

    fn parse_by_current_precedence(&mut self, precedence: u8) -> Result<Node, Error> {
        let mut node = self.parse_unary()?;
        while precedence < self.peeked.get_precedence() && self.peeked != Token::EOF {
            if let Token::Plus | Token::Minus | Token::Asterisk | Token::Slash = self.peeked {
                self.next();
                node = self.parse_binary(node)?;
            }
        }
        Ok(node)
    }

    fn parse_unary(&mut self) -> Result<Node, Error> {
        match self.current {
            Token::Minus => {
                let n = match self.peeked {
                    Token::Number(i) => i,
                    _ => panic!(),
                };
                self.next();
                Ok(Node::Number(-n))
            }
            Token::Number(n) => Ok(Node::Number(n)),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("{:?}", self.current),
            )),
        }
    }

    fn parse_binary(&mut self, l: Node) -> Result<Node, Error> {
        let o = self.current;
        self.next();
        let r = self.parse_by_current_precedence(o.get_precedence())?;
        Ok(Node::BinaryOperator(OperatorNode {
            op: o,
            left: Box::new(l),
            right: Box::new(r),
        }))
    }

    fn next(&mut self) {
        self.current = self.peeked;
        self.peeked = self.lx.next();
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input = String::from("1 * 2 + 3 * 4");
        let mut p = Parser::new(input);

        let node = p.parse();
        assert_eq!(
            node.unwrap(),
            Node::BinaryOperator(OperatorNode {
                op: Token::Plus,
                left: Box::new(Node::BinaryOperator(OperatorNode {
                    op: Token::Asterisk,
                    left: Box::new(Node::Number(1)),
                    right: Box::new(Node::Number(2))
                })),
                right: Box::new(Node::BinaryOperator(OperatorNode {
                    op: Token::Asterisk,
                    left: Box::new(Node::Number(3)),
                    right: Box::new(Node::Number(4))
                }))
            })
        )
    }
}

