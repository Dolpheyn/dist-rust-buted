pub mod client;
mod expression;
pub mod parse;

use anyhow::{anyhow, Result};
use thiserror::Error;
use tonic::transport::Channel;

use self::expression::{ExpressionTreeNode, Operator};
use crate::svc_mat::{
    add::client::AddClient, div::client::DivClient, gen::BinaryOpRequest, gen::MathResponse,
    mul::client::MulClient, sub::client::SubClient,
};

use futures::lock::Mutex;
use std::{future::Future, pin::Pin, sync::Arc};

pub const SERVICE_NAME: &str = "calc";
pub const SERVICE_HOST: &str = "[::1]";
pub const SERVICE_PORT: u32 = 50056;

type MathResult = Result<MathResponse>;

#[derive(Error, Debug)]
pub enum MathError {
    #[error("Invalid operand count for {operator:?} (expected 2, got {got:?})")]
    InvalidOperandCount { operator: Operator, got: usize },
    #[error("Client not supplied for operator {operator:?}")]
    ClientNotSupplied { operator: Operator },
}

type Shared<T> = Arc<Mutex<T>>;

#[derive(Default)]
pub struct MathOpClients {
    pub add: Option<Shared<AddClient<Channel>>>,
    pub sub: Option<Shared<SubClient<Channel>>>,
    pub mul: Option<Shared<MulClient<Channel>>>,
    pub div: Option<Shared<DivClient<Channel>>>,
}

#[derive(Default)]
pub struct Evaluator {
    clients: MathOpClients,
}

impl Evaluator {
    pub fn new(clients: MathOpClients) -> Self {
        Evaluator { clients }
    }

    // Evaluates a math expression
    pub fn eval<'a>(
        &'a self,
        expr: &'a expression::ExpressionTreeNode,
    ) -> Pin<Box<dyn Future<Output = MathResult> + Send + '_>> {
        match expr {
            ExpressionTreeNode::Val(n) => Box::pin(async { Ok(MathResponse { result: *n }) }),
            ExpressionTreeNode::Expr(expr) => Box::pin(self.eval_expr(expr)),
        }
    }

    async fn eval_expr(&self, expr: &expression::Expression) -> MathResult {
        let operand_count = expr.children.len();
        if expr.operator.is_binary() && operand_count != 2 {
            return Err(anyhow!(MathError::InvalidOperandCount {
                operator: expr.operator.clone(),
                got: operand_count,
            }));
        }

        match expr.operator {
            Operator::Add => {
                if self.clients.add.is_none() {
                    return Err(anyhow!(MathError::ClientNotSupplied {
                        operator: expr.operator.clone(),
                    }));
                }

                let add_client = Arc::clone(self.clients.add.as_ref().unwrap());
                let mut add_client = add_client.lock().await;

                let result = add_client
                    .add(BinaryOpRequest {
                        num1: self.eval(&expr.children[0]).await?.result,
                        num2: self.eval(&expr.children[1]).await?.result,
                    })
                    .await?
                    .into_inner();

                Ok(result)
            }
            Operator::Sub => unimplemented!(),
            Operator::Mul => unimplemented!(),
            Operator::Div => unimplemented!(),
        }
    }
}