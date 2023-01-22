use super::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    BinaryOperator(OperatorNode),
    Number(i32),
}

#[derive(Debug, PartialEq, Clone)]
pub struct OperatorNode {
    pub op: Token,
    pub left: Box<Node>,
    pub right: Box<Node>,
}