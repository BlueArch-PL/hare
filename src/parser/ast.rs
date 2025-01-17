
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
#[allow(dead_code)]
#[derive(Debug)]
pub enum AstNode {
    Program(Vec<AstNode>),
    /// 表示多个抽象语法树节点的块
    Block(Vec<AstNode>),
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
    /// 表示一个块执行后的返回值，包含返回值
    ReturnBlock(Box<AstNode>),
    /// 表示空节点，仅用于解析器内部使用，不会出现在对外的接口中
    Empty,

    // 流程控制节点
    /// If 节点，包含条件和分支，Fallback 分支
    If(
        Box<AstNode>,
        Box<AstNode>,
        Vec<AstNode>,
        Option<Box<AstNode>>,
    ),
    /// Elif 节点，包含条件和分支
    Elif(Box<AstNode>, Box<AstNode>),
    /// Else 节点，包含分支
    Else(Box<AstNode>),
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
            AstNode::Program(nodes) => {
                let mut code = String::new();
                for node in nodes {
                    code.push_str(&node.as_code());
                    code.push('\n');
                }
                code
            }
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
                    let type_annotation_string =
                        type_annotation.as_ref().unwrap().as_code().to_string();
                    let value_string = value.as_code().to_string();

                    format!(
                        "let {}: {} = {};",
                        identifier_string, type_annotation_string, value_string
                    )
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
            AstNode::Block(nodes) => {
                let mut code = String::new();

                for node in nodes {
                    code.push_str(&node.as_code());
                    code.push('\n');
                }

                format!("{{{}}}", code)
            }
            AstNode::ReturnBlock(value) => {
                let value_string = value.as_code().to_string();
                format!("return {};", value_string)
            }
            AstNode::If(cond, block, elif_nodes, else_nodes) => {
                let mut code = format!("if {} \n{}\n", cond.as_code(), block.as_code());
                for node in elif_nodes {
                    code.push_str(&node.as_code());
                    code.push('\n');
                }
                if let Some(else_node) = else_nodes {
                    code.push_str(&else_node.as_code());
                    code.push('\n');
                }
                code
            }
            AstNode::Elif(cond, block) => {
                format!("elif {} \n{}\n", cond.as_code(), block.as_code())
            }
            AstNode::Else(block) => {
                format!("else \n{}\n", block.as_code())
            }

            _ => "".to_string(),
        }
    }

    /// 将抽象语法树节点格式化，确保符合规范
    pub fn format_ast(&self) -> AstNode {
        match self {
            AstNode::Assign(identifier, type_annotation, value) => {
                let identifier = identifier.format_ast();
                let type_annotation = type_annotation
                    .as_ref()
                    .map(|node| Box::new(node.format_ast()));
                let value = value.format_ast();

                let value = match value {
                    AstNode::Expr(_, _, _) => value,
                    _ => AstNode::Expr(Box::new(value), None, None),
                };

                AstNode::Assign(Box::new(identifier), type_annotation, Box::new(value))
            }
            AstNode::Block(nodes) => {
                let mut formatted_nodes: Vec<AstNode> = vec![];
                for node in nodes {
                    let formatted_node = node.format_ast();
                    formatted_nodes.push(formatted_node);
                }

                AstNode::Block(formatted_nodes)
            }
            AstNode::ReturnBlock(value) => {
                let value = value.format_ast();
                AstNode::ReturnBlock(Box::new(value))
            }
            AstNode::SetValue(identifier, value) => {
                let identifier = identifier.format_ast();
                let value = value.format_ast();
                AstNode::SetValue(Box::new(identifier), Box::new(value))
            }
            AstNode::Expr(left, op, right) => {
                let left = left.format_ast();
                let right = right.as_ref().map(|node| Box::new(node.format_ast()));
                AstNode::Expr(Box::new(left), op.clone(), right)
            }
            AstNode::If(cond, block, elif_nodes, else_nodes) => {
                let cond = cond.format_ast();
                let block = block.format_ast();
                let elif_nodes = elif_nodes
                    .iter()
                    .map(|node| node.format_ast())
                    .collect::<Vec<AstNode>>();
                let else_node = else_nodes.as_ref().map(|node| Box::new(node.format_ast()));

                let cond = match cond {
                    AstNode::Expr(_, _, _) => cond,
                    _ => AstNode::Expr(Box::new(cond), None, None),
                };

                AstNode::If(Box::new(cond), Box::new(block), elif_nodes, else_node)
            }
            _ => self.clone(),
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

impl Clone for BinaryOp {
    /// 克隆二元操作符
    ///
    /// 这个方法克隆二元操作符，返回一个新的二元操作符。
    fn clone(&self) -> Self {
        match self {
            BinaryOp::Add => BinaryOp::Add,
            BinaryOp::Sub => BinaryOp::Sub,
            BinaryOp::Mul => BinaryOp::Mul,
            BinaryOp::Div => BinaryOp::Div,
            BinaryOp::Mod => BinaryOp::Mod,
            BinaryOp::Eq => BinaryOp::Eq,
            BinaryOp::Neq => BinaryOp::Neq,
            BinaryOp::Gt => BinaryOp::Gt,
            BinaryOp::Gte => BinaryOp::Gte,
            BinaryOp::Lt => BinaryOp::Lt,
            BinaryOp::Lte => BinaryOp::Lte,
        }
    }
}

impl Clone for AstNode {
    /// 克隆抽象语法树节点
    ///
    /// 这个方法克隆抽象语法树节点，返回一个新的抽象语法树节点。
    fn clone(&self) -> Self {
        use AstNode::*;
        match self {
            Program(nodes) => Program(nodes.iter().map(|node| node.clone()).collect()),
            Block(nodes) => Block(nodes.iter().map(|node| node.clone()).collect()),
            Constant(s) => Constant(s.clone()),
            Expr(left, op, right) => Expr(
                left.clone(),
                op.clone(),
                right.as_ref().map(|node| node.clone()),
            ),
            Identifier(s) => Identifier(s.clone()),
            Assign(id, value, expr) => Assign(
                id.clone(),
                value.as_ref().map(|node| node.clone()),
                expr.clone(),
            ),
            SetValue(func, params) => SetValue(func.clone(), params.clone()),
            ReturnBlock(expr) => ReturnBlock(expr.clone()),
            Empty => Empty,
            If(cond, block, elifs, fallback) => If(
                cond.clone(),
                block.clone(),
                elifs.iter().map(|node| node.clone()).collect(),
                fallback.as_ref().map(|node| node.clone()),
            ),
            Elif(cond, block) => Elif(cond.clone(), block.clone()),
            Else(block) => Else(block.clone()),
        }
    }
}
