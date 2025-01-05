use crate::BinaryOp;
use rand::Rng;

use crate::ast::AstNode;

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum CompilerError {
    #[error("Compile error: {0}")]
    CompileError(String),
}

/// 操作码枚举，用于表示字节码中的操作
#[derive(Debug)]
pub enum OpCode {
    // Stack
    /// 将一个或多个元素压入栈中
    Push,
    /// 取出栈顶的元素
    Pop,
    // Arithmetic
    /// 将栈顶的头两个元素取出相加，然后压入栈中
    Add,
    /// 将栈顶的头两个元素取出相减，然后压入栈中
    Sub,
    /// 将栈顶的头两个元素取出相乘，然后压入栈中
    Mul,
    /// 将栈顶的头两个元素取出相除，然后压入栈中
    Div,
    /// 将栈顶的头两个元素取出取模，然后压入栈中
    Mod,
    /// 将栈顶的元素取反，然后压入栈中
    Neg,
    // Comparison
    /// 比较栈顶的头两个元素是否相等，然后将结果压入栈中
    Eq,
    /// 比较栈顶的头两个元素是否不相等，然后将结果压入栈中
    Neq,
    /// 比较栈顶的头两个元素是否大于，然后将结果压入栈中
    Gt,
    /// 比较栈顶的头两个元素是否小于，然后将结果压入栈中
    Lt,
    /// 比较栈顶的头两个元素是否大于等于，然后将结果压入栈中
    Gte,
    /// 比较栈顶的头两个元素是否小于等于，然后将结果压入栈中
    Lte,
    // Flow
    /// 无条件跳转到指定的 Section，将该 Section 的返回值压入栈中
    Jump,
    /// 当栈顶的元素为 true 时跳转到指定的 Section，将该 Section 的返回值压入栈中
    JumpIf,
    // Heap
    /// 从堆中取出一个元素
    LoadName,
    /// 将栈顶的元素存入堆中
    StoreName,
    // Section
    /// 创建一个新的 Section，其名字为 args[0]，被放入堆中。
    /// 该指令执行后直到 EndMakeSection 指令被执行前，所有的指令都会被放入该 Section 中而不会被运行。
    MakeSection,
    /// 结束创建一个新的 Section
    EndMakeSection,
    /// 退出当前 Section
    Return,
}

pub struct ByteCode {
    pub op: OpCode,
    pub args: Vec<String>,
}

impl ByteCode {
    pub fn new(op: OpCode, args: Vec<String>) -> Self {
        Self { op, args }
    }
}

pub fn get_random_name() -> String {
    let mut rng = rand::thread_rng();
    let mut name = String::new();
    for _ in 0..8 {
        name.push(rng.gen_range(0..10).to_string().chars().next().unwrap());
    }
    name
}

impl AstNode {
    pub fn compile(&self) -> Result<Vec<ByteCode>, CompilerError> {
        let mut bytecode: Vec<ByteCode> = Vec::new();

        match self {
            AstNode::Program(nodes) => {
                for node in nodes {
                    bytecode.extend(node.compile()?);
                }
            }
            AstNode::Expr(left, op, right) => {
                let left_bytecode = left.compile()?;

                if op.is_some() {
                    let right_bytecode = right
                        .clone()
                        .ok_or(CompilerError::CompileError(
                            "Failed to compile right side of expression".to_string(),
                        ))?
                        .compile()?;

                    bytecode.extend(left_bytecode);
                    bytecode.extend(right_bytecode);
                    bytecode.push(ByteCode::new(op.clone().unwrap().to_opcode(), vec![]));
                } else {
                    bytecode.extend(left_bytecode);
                }
            }
            AstNode::Identifier(name) => {
                bytecode.push(ByteCode::new(OpCode::LoadName, vec![name.clone()]));
            }
            AstNode::Constant(name) => {
                bytecode.push(ByteCode::new(OpCode::Push, vec![name.clone()]));
            }
            _ => {}
        }

        Ok(bytecode)
    }
}

impl BinaryOp {
    pub fn to_opcode(&self) -> OpCode {
        match self {
            BinaryOp::Add => OpCode::Add,
            BinaryOp::Sub => OpCode::Sub,
            BinaryOp::Mul => OpCode::Mul,
            BinaryOp::Div => OpCode::Div,
            BinaryOp::Mod => OpCode::Mod,
            BinaryOp::Eq => OpCode::Eq,
            BinaryOp::Neq => OpCode::Neq,
            BinaryOp::Gt => OpCode::Gt,
            BinaryOp::Lt => OpCode::Lt,
            BinaryOp::Gte => OpCode::Gte,
            BinaryOp::Lte => OpCode::Lte,
        }
    }
}

pub fn print_bytecodes(codes: &Vec<ByteCode>) {
    for bytecode in codes {
        println!("{:?} {:?}", bytecode.op, bytecode.args);
    }
}
