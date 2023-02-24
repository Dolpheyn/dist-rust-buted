#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Div,
    Mul,
    Sub,
}

impl Operator {
    pub fn is_binary(&self) -> bool {
        match *self {
            Self::Add | Self::Div | Self::Mul | Self::Sub => true,
        }
    }
}

// An expression tree's node is either a value(number), or an expression.
#[derive(Debug, PartialEq)]
pub enum ExpressionTreeNode {
    Val(i32),
    Expr(Expression),
}

// An expression is something that can be evaluated to be a value depending on its
// operator and children.
//
// If a child is also an Expression::Expr, it needs to be evaluated into a value, before we can
// complete the parent expression.
#[derive(Debug, PartialEq)]
pub struct Expression {
    pub operator: Operator,
    pub children: Vec<ExpressionTreeNode>,
}
