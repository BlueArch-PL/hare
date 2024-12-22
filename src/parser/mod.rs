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

pub fn parse_constant(pair: Pair<Rule>) -> Option<AstNode> {
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
        Rule::constant => Some(parse_constant(pair.into_inner().next().unwrap()).unwrap()),
        _ => None,
    }
}

pub fn parse_ident(pair: Pair<Rule>) -> Option<AstNode> {
    match pair.as_rule() {
        Rule::ident => Some(AstNode::Identifier(pair.as_str().to_string())),
        _ => None,
    }
}

pub fn parse_expr(pair: Pair<Rule>) -> Option<AstNode> {
    PRATT_PARSER
        .map_primary(|primary: Pair<'_, Rule>| {
            let mut pair = parse_constant(primary.clone());

            if pair.is_none() {
                pair = parse_ident(primary.clone());
            }

            if pair.is_none() {
                pair = parse_expr(primary.clone());
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
        .parse(pair.into_inner())
}

pub fn parse_pairs(
    pairs: Pairs<Rule>,
    level: Option<u8>,
) -> Result<Vec<AstNode>, pest::error::Error<Rule>> {
    let mut ast_nodes: Vec<AstNode> = vec![];
    for pair in pairs {
        print_pair(&pair, level);

        match pair.as_rule() {
            Rule::expr => ast_nodes.push(parse_expr(pair).unwrap()),
            // Constant nodes
            Rule::constant => ast_nodes.push(parse_constant(pair).unwrap()),
            // Identifier nodes
            Rule::ident => ast_nodes.push(parse_ident(pair).unwrap()),
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

pub fn parse(input: &str) -> Result<Vec<AstNode>, pest::error::Error<Rule>> {
    let pairs = BlueArchParser::parse(Rule::program, input)?;
    parse_pairs(pairs, Some(0))
}
