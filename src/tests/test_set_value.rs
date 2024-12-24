use crate::parser::ast::AstNode::*;
use crate::parser::ast::BinaryOp::*;
use crate::parser::BlueArchParser;
use crate::Rule;
use crate::parser::parse_pairs;
use pest::Parser;

#[test]
fn test_set_value1() {
    let expr = "a = 1";
    let pairs = BlueArchParser::parse(Rule::program, expr);

    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert!(ast.len() == 1);
    assert_eq!(ast[0].as_code(), "a = 1;");
}

#[test]
fn test_set_value2() {
    let expr = "a = 1 + 2";
    let pairs = BlueArchParser::parse(Rule::program, expr);
    assert!(pairs.is_ok());
    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    assert!(ast.len() == 1);
    assert_eq!(ast[0].as_code(), "a = (1 + 2);");
}

#[test]
fn test_set_value3() {
    let expr = "a = 1 + 2 * 3";
    let pairs = BlueArchParser::parse(Rule::program, expr);
    assert!(pairs.is_ok());
    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap(); 
    assert!(ast.len() == 1);
    assert_eq!(ast[0].as_code(), "a = (1 + (2 * 3));");
}