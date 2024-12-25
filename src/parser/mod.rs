pub mod ast;
pub mod errors;
pub mod grammar;
pub mod utils;

use errors::ParserError;
use pest::iterators::{Pair, Pairs};
use pest::pratt_parser::PrattParser;
use pest::Parser;
use utils::*;

pub use ast::*;
pub use grammar::{BlueArchParser, Rule};

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};

        PrattParser::<Rule>::new()
            .op(Op::infix(Rule::equals, Left)
                | Op::infix(Rule::not_equals, Left)
                | Op::infix(Rule::greater_than, Left)
                | Op::infix(Rule::greater_than_or_equal_to, Left)
                | Op::infix(Rule::less_than, Left)
                | Op::infix(Rule::less_than_or_equal_to, Left))
            .op(Op::infix(Rule::add, Left) | Op::infix(Rule::subtract, Left))
            .op(Op::infix(Rule::multiply, Left) | Op::infix(Rule::divide, Left))
            .op(Op::infix(Rule::modulo, Left))
    };
}

pub fn parse_binary_op(pair: &Pair<Rule>) -> Result<BinaryOp, ParserError> {
    match pair.as_rule() {
        Rule::add => Some(BinaryOp::Add),
        Rule::subtract => Some(BinaryOp::Sub),
        Rule::multiply => Some(BinaryOp::Mul),
        Rule::divide => Some(BinaryOp::Div),
        Rule::modulo => Some(BinaryOp::Mod),
        Rule::equals => Some(BinaryOp::Eq),
        Rule::not_equals => Some(BinaryOp::Neq),
        Rule::greater_than => Some(BinaryOp::Gt),
        Rule::greater_than_or_equal_to => Some(BinaryOp::Gte),
        Rule::less_than => Some(BinaryOp::Lt),
        Rule::less_than_or_equal_to => Some(BinaryOp::Lte),
        _ => None,
    }
    .ok_or(ParserError::SyntaxError(format!(
        "Unknown binary operator: {:?}",
        pair.as_rule()
    )))
}

/// 解析表达式节点
///
/// 该函数接受一个`Pair<Rule>`类型的参数，根据其规则解析为相应的表达式节点。
///
/// # 参数
///
/// - `pair`: 一个`Pair<Rule>`类型的参数，表示要解析的表达式节点。
///
/// # 返回值
///
/// 返回一个`Option<AstNode>`类型的值，表示解析后的表达式节点。如果解析失败，则返回`None`。
pub fn parse_expr(pair: &Pair<Rule>) -> Option<AstNode> {
    PRATT_PARSER
        .map_primary(|primary: Pair<'_, Rule>| {
            let mut pair = parse_pair(&primary).ok();

            pair
        })
        .map_infix(|lhs, op, rhs| {
            let left = lhs.unwrap();
            let right = rhs;

            Some(AstNode::Expr(
                Box::new(left),
                match op.as_rule() {
                    Rule::add => Some(BinaryOp::Add),
                    Rule::subtract => Some(BinaryOp::Sub),
                    Rule::multiply => Some(BinaryOp::Mul),
                    Rule::divide => Some(BinaryOp::Div),
                    Rule::modulo => Some(BinaryOp::Mod),
                    Rule::equals => Some(BinaryOp::Eq),
                    Rule::not_equals => Some(BinaryOp::Neq),
                    Rule::greater_than => Some(BinaryOp::Gt),
                    Rule::greater_than_or_equal_to => Some(BinaryOp::Gte),
                    Rule::less_than => Some(BinaryOp::Lt),
                    Rule::less_than_or_equal_to => Some(BinaryOp::Lte),
                    _ => None,
                },
                Some(Box::new(right.unwrap())),
            ))
        })
        .parse(pair.clone().into_inner())
        .map(|x| x.format_ast())
}

pub fn parse_pair(pair: &Pair<Rule>) -> Result<AstNode, ParserError> {
    match pair.as_rule() {
        Rule::expr => parse_expr(&pair).ok_or(ParserError::SyntaxError(format!(
            "Invalid expression: {:?}",
            pair.as_str()
        ))),
        // 常量节点
        Rule::int => Ok(AstNode::Constant(
            pair.as_str().parse::<i128>().unwrap().to_string(),
        )),
        Rule::float => Ok(AstNode::Constant(
            pair.as_str().parse::<f64>().unwrap().to_string(),
        )),
        Rule::string => Ok(AstNode::Constant(pair.as_str().to_string())),
        Rule::boolean => Ok(AstNode::Constant(
            pair.as_str().parse::<bool>().unwrap().to_string(),
        )),
        // 标识符
        Rule::ident => Ok(AstNode::Identifier(pair.as_str().to_string())),
        // 语句
        Rule::assign_statement => {
            let mut pairs = pair.clone().into_inner();
            let identifier = parse_pair(&pairs.next().unwrap())?;
            let mut type_annotation: Option<Box<AstNode>> = None;
            let value: AstNode;

            if pairs.len() == 1 {
                value = parse_pair(&pairs.next().unwrap())?;
            } else {
                type_annotation = Some(Box::new(parse_pair(&pairs.next().unwrap())?));
                value = parse_pair(&pairs.next().unwrap())?;
            };

            Ok(AstNode::Assign(
                Box::new(identifier),
                type_annotation,
                Box::new(value),
            ))
        }
        Rule::set_value_statement => {
            let mut pairs = pair.clone().into_inner();
            let identifier = parse_pair(&pairs.next().unwrap())?;
            let value = parse_pair(&pairs.next().unwrap())?;
            Ok(AstNode::SetValue(Box::new(identifier), Box::new(value)))
        }
        Rule::return_block_statement => {
            let mut pairs = pair.clone().into_inner();
            let value = parse_pair(&pairs.next().unwrap())?;
            Ok(AstNode::ReturnBlock(Box::new(value)))
        }
        Rule::if_statement => {
            let mut pairs = pair.clone().into_inner();
            let condition = parse_pair(&pairs.next().unwrap())?;
            let block = parse_pair(&pairs.next().unwrap())?;
            let mut elif_branches: Vec<AstNode> = vec![];
            let mut else_branch: Option<Box<AstNode>> = None;

            for pair in pairs {
                let node = parse_pair(&pair)?;

                match node {
                    AstNode::Elif(_, _) => elif_branches.push(node),
                    AstNode::Else(_) => else_branch = Some(Box::new(node)),
                    _ => {
                        return Err(ParserError::UnknownError(format!(
                            "Unknown pair in if statement: {:?}",
                            pair.as_rule()
                        )));
                    }
                }
            }

            Ok(AstNode::If(
                Box::new(condition),
                Box::new(block),
                elif_branches,
                else_branch,
            ))
        }
        Rule::elif_statement => {
            let mut pairs = pair.clone().into_inner();
            let condition = parse_pair(&pairs.next().unwrap())?;
            let block = parse_pair(&pairs.next().unwrap())?;
            Ok(AstNode::Elif(Box::new(condition), Box::new(block)))
        }
        Rule::else_statement => {
            let mut pairs = pair.clone().into_inner();
            let block = parse_pair(&pairs.next().unwrap())?;
            Ok(AstNode::Else(Box::new(block)))
        }
        // 块
        Rule::block => Ok(AstNode::Block(parse_pairs(pair.clone().into_inner())?)),
        // 递归解析
        Rule::statement | Rule::constant => parse_pair(&pair.clone().into_inner().next().ok_or(
            ParserError::UnknownError(format!("Unknown pair: {:?}", pair.as_rule())),
        )?),
        // 其他
        Rule::EOI | Rule::COMMENT => Ok(AstNode::Empty),
        _ => Err(ParserError::UnknownError(format!(
            "Unknown pair: {:?}",
            pair.as_rule()
        ))),
    }
    .map(|x| x.format_ast())
}

/// 解析节点对列表
///
/// 该函数接受一个`Pairs<Rule>`类型的参数，根据其规则解析为相应的 Ast Node。
///
/// # 参数
///
/// - `pairs`: 一个`Pairs<Rule>`类型的参数，表示要解析的节点对。
///
/// # 返回值
///
/// 返回一个`Result<Vec<AstNode>, pest::error::Error<Rule>>`类型的值，表示解析后的节点对。如果解析失败，则返回`Err`。
pub fn parse_pairs(pairs: Pairs<Rule>) -> Result<Vec<AstNode>, ParserError> {
    let mut ast_nodes: Vec<AstNode> = vec![];

    for pair in pairs {
        let node = parse_pair(&pair)?;
        if let AstNode::Empty = node {
            continue;
        }

        ast_nodes.push(node);
    }

    Ok(ast_nodes)
}

/// 解析输入字符串
///
/// 该函数接受一个`&str`类型的参数，根据其规则解析为相应的节点对。
///
/// # 参数
///
/// - `input`: 一个`&str`类型的参数，表示要解析的输入字符串。
///
/// # 返回值
///
/// 返回一个`Result<Vec<AstNode>, pest::error::Error<Rule>>`类型的值，表示解析后的节点对。如果解析失败，则返回`Err`。
pub fn parse(input: &str) -> Result<Vec<AstNode>, ParserError> {
    let pairs = BlueArchParser::parse(Rule::program, input)?;
    print_pairs(&pairs, None);

    parse_pairs(pairs)
}
