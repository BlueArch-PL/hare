#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Debug)]
pub enum AstNode {
    Constant(String),
    Expr(Box<AstNode>, Option<BinaryOp>, Option<Box<AstNode>>),
    Identifier(String),
    Empty,
    PrefixNotation(Vec<String>),
}

impl BinaryOp {
    pub fn as_raw(&self) -> &str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::Eq => "==",
            BinaryOp::Neq => "!=",
            BinaryOp::Gt => ">",
            BinaryOp::Gte => ">=",
            BinaryOp::Lt => "<",
            BinaryOp::Lte => "<=",
        }
    }
}

impl AstNode {
    pub fn as_code(&self) -> String {
        match self {
            AstNode::Constant(s) => s.to_string(),
            AstNode::Identifier(s) => s.to_string(),
            AstNode::Expr(left, op, right) => {
                let left_string = left.as_code().to_string();

                if op.is_some() && right.is_some() {
                    let right_string = right.as_ref().unwrap().as_code().to_string();

                    format!(
                        "({} {} {})",
                        left_string,
                        op.as_ref().unwrap().as_raw(),
                        right_string
                    )
                } else {
                    left_string
                }
            }
            _ => "".to_string(),
        }
    }
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
