use crate::svc_mat::calc::expression::ExpressionTreeNode;

use super::expression::{Expression, Operator};

pub fn parse<S: Into<String>>(source: S) -> Option<ExpressionTreeNode> {
    let mut parser = Parser::new(source);
    parser.parse()
}

pub struct Parser {
    // col is 0-indexed
    col: usize,
    input: String,
}

impl Parser {
    pub fn new(source: impl Into<String>) -> Self {
        return Self {
            col: 0,
            input: source.into(),
        };
    }

    pub fn parse(&mut self) -> Option<ExpressionTreeNode> {
        let mut ast = None;

        if self.is_eol() {
            return ast;
        }

        // Skip whitespaces
        self.take_while(is_whitespace);

        if self.peek().is_numeric() {
            let number = self.take_while(is_numeric);
            dbg!(&number);
            ast = Some(ExpressionTreeNode::Val(number.parse().unwrap()));
        } else {
            let maybe_operator = self.take();
            let operator = match maybe_operator {
                '+' => Operator::Add,
                '-' => Operator::Sub,
                '*' => Operator::Mul,
                '/' => Operator::Div,
                _ => panic!("unexpected character found while parsing"),
            };
            ast = Some(ExpressionTreeNode::Expr(Expression {
                operator,
                children: vec![self.parse().unwrap(), self.parse().unwrap()],
            }));
        }

        ast
    }

    fn is_eol(&self) -> bool {
        self.col == self.input.len()
    }

    fn peek(&self) -> char {
        // We make sure column won't go pass the input length by not mutating the input, and check
        // for eol.
        self.input.chars().nth(self.col).unwrap()
    }

    fn take(&mut self) -> char {
        let ret = self.input.chars().nth(self.col).unwrap();
        self.col += 1;
        ret
    }

    fn take_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut ret = String::new();
        while !self.is_eol() && test(self.peek()) {
            ret.push(self.take());
        }

        ret
    }
}

#[inline]
fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

#[inline]
fn is_numeric(c: char) -> bool {
    c.is_numeric()
}

#[cfg(test)]
mod test {
    use crate::svc_mat::calc::expression::{Expression, ExpressionTreeNode, Operator};

    use super::parse;

    #[test]
    fn it_parses() {
        let input = "+ 1 - 3 4";
        let res = parse(input);

        assert!(res.is_some());

        let res = res.unwrap();
        assert_eq!(
            res,
            ExpressionTreeNode::Expr(Expression {
                operator: Operator::Add,
                children: vec![
                    ExpressionTreeNode::Val(1),
                    ExpressionTreeNode::Expr(Expression {
                        operator: Operator::Sub,
                        children: vec![ExpressionTreeNode::Val(3), ExpressionTreeNode::Val(4)]
                    })
                ]
            })
        );
    }
}
