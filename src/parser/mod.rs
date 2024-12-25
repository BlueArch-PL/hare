pub mod ast;
pub mod grammar;
pub mod utils;

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

/// 解析常量节点
///
/// 该函数接受一个`Pair<Rule>`类型的参数，根据其规则解析为相应的常量节点。
///
/// # 参数
///
/// - `pair`: 一个`Pair<Rule>`类型的参数，表示要解析的常量节点。
///
/// # 返回值
///
/// 返回一个`Option<AstNode>`类型的值，表示解析后的常量节点。如果解析失败，则返回`None`。
pub fn parse_constant(pair: &Pair<Rule>) -> Option<AstNode> {
    match pair.as_rule() {
        Rule::int => Some(AstNode::Constant(
            pair.as_str().parse::<i128>().unwrap().to_string(),
        )),
        Rule::float => Some(AstNode::Constant(
            pair.as_str().parse::<f64>().unwrap().to_string(),
        )),
        Rule::string => Some(AstNode::Constant(pair.as_str().to_string())),
        Rule::boolean => Some(AstNode::Constant(
            pair.as_str().parse::<bool>().unwrap().to_string(),
        )),
        Rule::constant => Some(parse_constant(&pair.clone().into_inner().next().unwrap()).unwrap()),
        _ => None,
    }
}

/// 解析标识符节点
///
/// 该函数接受一个`Pair<Rule>`类型的参数，根据其规则解析为相应的标识符节点。
///
/// # 参数
///
/// - `pair`: 一个`Pair<Rule>`类型的参数，表示要解析的标识符节点。
///
/// # 返回值
///
/// 返回一个`Option<AstNode>`类型的值，表示解析后的标识符节点。如果解析失败，则返回`None`。
pub fn parse_ident(pair: &Pair<Rule>) -> Option<AstNode> {
    match pair.as_rule() {
        Rule::ident => Some(AstNode::Identifier(pair.as_str().to_string())),
        _ => None,
    }
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
            let mut pair = parse_constant(&primary);

            if pair.is_none() {
                pair = parse_ident(&primary);
            }

            if pair.is_none() {
                pair = parse_expr(&primary);
            }

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
}

/// 解析语句节点
///
/// 该函数接受一个`Pair<Rule>`类型的参数，根据其规则解析为相应的语句节点。
///
/// # 参数
///
/// - `pair`: 一个`Pair<Rule>`类型的参数，表示要解析的语句节点。
///
/// # 返回值
///
/// 返回一个`Option<AstNode>`类型的值，表示解析后的语句节点。如果解析失败，则返回`None`。
pub fn parse_statement(pair: &Pair<Rule>) -> Option<AstNode> {
    match pair.as_rule() {
        Rule::assign_statement => {
            let mut pairs = pair.clone().into_inner();
            let identifier = pairs.next().unwrap();

            // have type annotation
            if pair.clone().into_inner().count() == 3 {
                let type_annotation = pairs.next().unwrap();
                let value = pairs.next().unwrap();

                Some(AstNode::Assign(
                    Box::new(parse_ident(&identifier).unwrap()),
                    Some(Box::new(parse_ident(&type_annotation).unwrap())),
                    Box::new(parse_expr(&value).unwrap()),
                ))
            } else if pair.clone().into_inner().count() == 2 {
                let value = pairs.next().unwrap();

                Some(AstNode::Assign(
                    Box::new(parse_ident(&identifier).unwrap()),
                    None,
                    Box::new(parse_expr(&value).unwrap()),
                ))
            } else {
                panic!("Invalid assign statement")
            }
        }
        Rule::set_value_statement => {
            let mut pairs = pair.clone().into_inner();
            let identifier = pairs.next().unwrap();
            let value = pairs.next().unwrap();

            let identifier = parse_ident(&identifier);
            let value = parse_expr(&value);

            Some(AstNode::SetValue(
                Box::new(identifier.expect("Invalid identifier")),
                Box::new(value.expect("Invalid value")),
            ))
        }
        Rule::return_block_statement => {
            let mut pairs = pair.clone().into_inner();

            Some(AstNode::ReturnBlock(Box::new(
                parse_expr(&pairs.next().unwrap()).unwrap(),
            )))
        }
        Rule::statement => parse_statement(&pair.clone().into_inner().next().unwrap()),
        Rule::expr => parse_expr(pair),
        _ => None,
    }
}

/// 解析节点对
///
/// 该函数接受一个`Pairs<Rule>`类型的参数，根据其规则解析为相应的 Ast Node。
///
/// # 参数
///
/// - `pairs`: 一个`Pairs<Rule>`类型的参数，表示要解析的节点对。
/// - `level`: 一个`Option<u8>`类型的参数，表示解析的层级。
///
/// # 返回值
///
/// 返回一个`Result<Vec<AstNode>, pest::error::Error<Rule>>`类型的值，表示解析后的节点对。如果解析失败，则返回`Err`。
pub fn parse_pairs(
    pairs: Pairs<Rule>,
    level: Option<u8>,
) -> Result<Vec<AstNode>, pest::error::Error<Rule>> {
    let mut ast_nodes: Vec<AstNode> = vec![];
    for pair in pairs {
        print_pair(&pair, level);

        match pair.as_rule() {
            Rule::block => ast_nodes.push(AstNode::Block(parse_pairs(
                pair.into_inner(),
                Some(level.unwrap_or(0) + 1),
            )?)),
            // Statement nodes
            Rule::statement => ast_nodes.push(parse_statement(&pair).unwrap()),
            // Constant nodes
            Rule::constant => ast_nodes.push(parse_constant(&pair).unwrap()),
            // Identifier nodes
            Rule::ident => ast_nodes.push(parse_ident(&pair).unwrap()),
            // Other nodes
            Rule::EOI => {}
            _ => {
                let inner_nodes = parse_pairs(pair.into_inner(), Some(level.unwrap() + 1)).unwrap();

                for node in inner_nodes {
                    ast_nodes.push(node);
                }
            }
        };
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
pub fn parse(input: &str) -> Result<Vec<AstNode>, pest::error::Error<Rule>> {
    let pairs = BlueArchParser::parse(Rule::program, input)?;
    parse_pairs(pairs, Some(0))
}
