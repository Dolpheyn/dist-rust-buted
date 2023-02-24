pub mod client;
mod expression;
pub mod parse;

use anyhow::{anyhow, Result};
use thiserror::Error;

use self::expression::{ExpressionTreeNode, Operator};
use crate::svc_mat::{self, gen::BinaryOpRequest, gen::MathResponse};

use std::{future::Future, pin::Pin};

pub const SERVICE_NAME: &str = "calc";
pub const SERVICE_HOST: &str = "[::1]";
pub const SERVICE_PORT: u32 = 50056;

type MathResult = Result<MathResponse>;

pub fn parse<S: Into<String>>(source: S) -> Option<ExpressionTreeNode> {
    let mut parser = parse::Parser::new(source);
    parser.parse()
}

#[derive(Error, Debug)]
pub enum MathError {
    #[error("Invalid operand count for {operator:?} (expected 2, got {got:?})")]
    InvalidOperandCount { operator: Operator, got: usize },
    #[error("Unable to connect to operator {operator:?}'s server")]
    OperatorServerUnreachable { operator: Operator },
}

// Evaluates a math expression
pub fn eval<'a>(
    expr: &'a expression::ExpressionTreeNode,
) -> Pin<Box<dyn Future<Output = MathResult> + Send + '_>> {
    match expr {
        ExpressionTreeNode::Val(n) => Box::pin(async { Ok(MathResponse { result: *n }) }),
        ExpressionTreeNode::Expr(expr) => Box::pin(eval_expr(expr)),
    }
}

async fn eval_expr(expr: &expression::Expression) -> MathResult {
    let operand_count = expr.children.len();
    if expr.operator.is_binary() && operand_count != 2 {
        return Err(anyhow!(MathError::InvalidOperandCount {
            operator: expr.operator.clone(),
            got: operand_count,
        }));
    }

    match expr.operator {
        Operator::Add => {
            let mut add_client = match svc_mat::add::client::client().await {
                Ok(client) => client,
                Err(_) => {
                    return Err(anyhow!(MathError::OperatorServerUnreachable {
                        operator: expr.operator.clone()
                    }));
                }
            };

            let result = add_client
                .add(BinaryOpRequest {
                    num1: eval(&expr.children[0]).await?.result,
                    num2: eval(&expr.children[1]).await?.result,
                })
                .await?
                .into_inner();

            Ok(result)
        }
        Operator::Sub => {
            let mut sub_client = match svc_mat::sub::client::client().await {
                Ok(client) => client,
                Err(_) => {
                    return Err(anyhow!(MathError::OperatorServerUnreachable {
                        operator: expr.operator.clone()
                    }));
                }
            };
            let result = sub_client
                .sub(BinaryOpRequest {
                    num1: eval(&expr.children[0]).await?.result,
                    num2: eval(&expr.children[1]).await?.result,
                })
                .await?
                .into_inner();

            Ok(result)
        }
        Operator::Mul => {
            let mut mul_client = match svc_mat::mul::client::client().await {
                Ok(client) => client,
                Err(_) => {
                    return Err(anyhow!(MathError::OperatorServerUnreachable {
                        operator: expr.operator.clone()
                    }));
                }
            };
            let result = mul_client
                .mul(BinaryOpRequest {
                    num1: eval(&expr.children[0]).await?.result,
                    num2: eval(&expr.children[1]).await?.result,
                })
                .await?
                .into_inner();

            Ok(result)
        }
        Operator::Div => {
            let mut div_client = match svc_mat::div::client::client().await {
                Ok(client) => client,
                Err(_) => {
                    return Err(anyhow!(MathError::OperatorServerUnreachable {
                        operator: expr.operator.clone()
                    }));
                }
            };
            let result = div_client
                .div(BinaryOpRequest {
                    num1: eval(&expr.children[0]).await?.result,
                    num2: eval(&expr.children[1]).await?.result,
                })
                .await?
                .into_inner();

            Ok(result)
        }
    }
}
