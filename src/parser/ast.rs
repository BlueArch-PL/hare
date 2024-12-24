/// 表示二元操作符的枚举类型
///
/// 这个枚举类型定义了所有支持的二元操作符，如加法、减法、乘法、除法等。
#[derive(Debug)]
pub enum BinaryOp {
    /// 加法操作符
    Add,
    /// 减法操作符
    Sub,
    /// 乘法操作符
    Mul,
    /// 除法操作符
    Div,
    /// 取模操作符
    Mod,
    /// 等于操作符
    Eq,
    /// 不等于操作符
    Neq,
    /// 大于操作符
    Gt,
    /// 大于等于操作符
    Gte,
    /// 小于操作符
    Lt,
    /// 小于等于操作符
    Lte,
}

/// 表示抽象语法树节点的枚举类型
///
/// 这个枚举类型定义了所有可能的抽象语法树节点，如常量、表达式、标识符、赋值语句等。
#[derive(Debug)]
pub enum AstNode {
    /// 表示常量的节点
    Constant(String),
    /// 表示表达式的节点，包含左操作数、操作符和右操作数
    Expr(Box<AstNode>, Option<BinaryOp>, Option<Box<AstNode>>),
    /// 表示标识符的节点
    Identifier(String),
    /// 表示赋值语句的节点，包含标识符、类型注解和值
    Assign(Box<AstNode>, Option<Box<AstNode>>, Box<AstNode>),
    /// 表示设置变量值的节点，包含函数名、参数列表和函数体
    SetValue(Box<AstNode>, Box<AstNode>),
    /// 表示空节点
    Empty,
}

impl BinaryOp {
    /// 返回二元操作符的原始字符串表示
    ///
    /// 这个方法返回二元操作符的原始字符串表示，如 "+"、"-"、"*" 等。
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
    /// 返回抽象语法树节点的代码表示
    ///
    /// 这个方法返回抽象语法树节点的代码表示，如常量、表达式、标识符、赋值语句等。
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
            AstNode::Assign(identifier, type_annotation, value) => {
                let identifier_string = identifier.as_code().to_string();
                
                if type_annotation.is_some() {
                    let type_annotation_string = type_annotation.as_ref().unwrap().as_code().to_string();
                    let value_string = value.as_code().to_string();

                    format!("let {}: {} = {};", identifier_string, type_annotation_string, value_string)
                } else {
                    let value_string = value.as_code().to_string();

                    format!("let {} = {};", identifier_string, value_string)
                }
            }
            AstNode::SetValue(identifier, value) => {
                let identifier_string = identifier.as_code().to_string();
                let value_string = value.as_code().to_string();

                format!("{} = {};", identifier_string, value_string)
            }
            _ => "".to_string(),
        }
    }
}

impl PartialEq for BinaryOp {
    /// 比较两个二元操作符是否相等
    ///
    /// 这个方法比较两个二元操作符是否相等，返回 `true` 或 `false`。
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
    /// 比较两个抽象语法树节点是否相等
    ///
    /// 这个方法比较两个抽象语法树节点是否相等，返回 `true` 或 `false`。
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
