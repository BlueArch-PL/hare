#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
}

#[derive(Debug)]
pub enum AstNode {
    Constant(String),
    Expr(Box<AstNode>, Option<BinaryOp>, Option<Box<AstNode>>),
    Empty,
}

impl PartialEq for BinaryOp {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BinaryOp::Add, BinaryOp::Add) => true,
            (BinaryOp::Sub, BinaryOp::Sub) => true,
            (BinaryOp::Mul, BinaryOp::Mul) => true,
            (BinaryOp::Div, BinaryOp::Div) => true,
            (BinaryOp::Mod, BinaryOp::Mod) => true,
            (BinaryOp::Eq, BinaryOp::Eq) => true,
            _ => false,
        }
    }
}

impl PartialEq for AstNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AstNode::Constant(a), AstNode::Constant(b)) => a == b,
            (AstNode::Expr(a, op1, b), AstNode::Expr(c, op2, d)) => {
                a == c
                    && op1.as_ref().unwrap_or(&BinaryOp::Add)
                        == op2.as_ref().unwrap_or(&BinaryOp::Add)
                    && b == d
            }
            _ => false,
        }
    }
}
