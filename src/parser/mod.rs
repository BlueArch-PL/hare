pub mod ast;
pub mod grammar;
mod utils;

use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use utils::*;

pub use ast::*;
pub use grammar::{BlueArchParser, Rule};

pub fn parse_pairs(pairs: Pairs<Rule>, level: Option<u8>) -> Result<Vec<AstNode>, String> {
    let mut ast_nodes: Vec<AstNode> = vec![];
    for pair in pairs {
        print_pair(&pair, level);

        match pair.as_rule() {
            Rule::expr => {
                use pest::pratt_parser::{Assoc::*, Op};

                let pratt_parser = PrattParser::<Rule>::new()
                    .op(Op::infix(Rule::add, Left) | Op::infix(Rule::subtract, Left))
                    .op(Op::infix(Rule::multiply, Left) | Op::infix(Rule::divide, Left))
                    .op(Op::infix(Rule::modulo, Left))
                    .op(Op::infix(Rule::equals, Left));

                let pairs = pratt_parser
                    .map_primary(|primary| {
                        parse_pairs(primary.into_inner(), Some(level.unwrap_or(0) + 1))
                            .unwrap()
                            .pop()
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
                                _ => None,
                            },
                            Some(Box::new(right.unwrap())),
                        ))
                    })
                    .parse(pair.into_inner())
                    .unwrap();

                ast_nodes.push(pairs);
            }
            // Constant nodes
            Rule::int => ast_nodes.push(AstNode::Constant(
                pair.as_str().parse::<i128>().unwrap().to_string(),
            )),
            Rule::float => ast_nodes.push(AstNode::Constant(
                pair.as_str().parse::<f64>().unwrap().to_string(),
            )),
            Rule::string => ast_nodes.push(AstNode::Constant(pair.as_str().to_string())),
            Rule::boolean => ast_nodes.push(AstNode::Constant(
                pair.as_str().parse::<bool>().unwrap().to_string(),
            )),
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

pub fn parse(input: &str) -> Result<Vec<AstNode>, String> {
    let pairs = BlueArchParser::parse(Rule::program, input);
    parse_pairs(pairs.unwrap(), Some(0))
}
