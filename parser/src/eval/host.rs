use crate::parse::{ast::Node, token::Token};


pub fn eval(ast: Node) -> i32 {
    match ast {
        Node::Number(num) => num,
        Node::BinaryOperator(bo) => match bo.op {
            Token::Plus => eval(*bo.left) + eval(*bo.right),
            Token::Minus => eval(*bo.left) - eval(*bo.right),
            Token::Asterisk => eval(*bo.left) * eval(*bo.right),
            Token::Slash => eval(*bo.left) / eval(*bo.right),
            _ => unreachable!(),
        },
    }
}