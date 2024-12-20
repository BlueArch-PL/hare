use crate::parser::*;
use ast::AstNode::*;
use ast::BinaryOp::*;
use pest::Parser;

#[test]
fn test_expr1() {
    let expr = "1 + 2;";
    let pairs = BlueArchParser::parse(Rule::program, expr);

    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0],
        AstNode::Expr(
            Box::new(AstNode::Constant("1".to_string())),
            Some(BinaryOp::Add),
            Some(Box::new(AstNode::Constant("2".to_string())))
        )
    );
}

#[test]
fn test_expr2() {
    let expr = "1 * 3;";
    let pairs = BlueArchParser::parse(Rule::program, expr);
    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0],
        AstNode::Expr(
            Box::new(AstNode::Constant("1".to_string())),
            Some(BinaryOp::Mul),
            Some(Box::new(AstNode::Constant("3".to_string())))
        )
    );
}

#[test]
fn test_expr3() {
    let expr = "1 - 2;";
    let pairs = BlueArchParser::parse(Rule::program, expr);
    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0],
        AstNode::Expr(
            Box::new(AstNode::Constant("1".to_string())),
            Some(BinaryOp::Sub),
            Some(Box::new(AstNode::Constant("2".to_string())))
        )
    );
}

#[test]
fn test_expr4() {
    let expr = "1 / 2;";
    let pairs = BlueArchParser::parse(Rule::program, expr);
    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0],
        AstNode::Expr(
            Box::new(AstNode::Constant("1".to_string())),
            Some(BinaryOp::Div),
            Some(Box::new(AstNode::Constant("2".to_string())))
        )
    );
}

#[test]
fn test_expr5() {
    let expr = "1 % 2;";
    let pairs = BlueArchParser::parse(Rule::program, expr);
    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0],
        AstNode::Expr(
            Box::new(AstNode::Constant("1".to_string())),
            Some(BinaryOp::Mod),
            Some(Box::new(AstNode::Constant("2".to_string())))
        )
    );
}

#[test]
fn test_expr6() {
    let expr = "1 == 2;";
    let pairs = BlueArchParser::parse(Rule::program, expr);
    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0],
        AstNode::Expr(
            Box::new(AstNode::Constant("1".to_string())),
            Some(BinaryOp::Eq),
            Some(Box::new(AstNode::Constant("2".to_string())))
        )
    );
}

#[test]
fn test_expr7() {
    let expr = "1 + 2 * 3 / 4;";
    let pairs = BlueArchParser::parse(Rule::program, expr);
    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0],
        Expr(
            Box::new(Constant("1".to_string())),
            Some(Add,),
            Some(Box::new(Expr(
                Box::new(Expr(
                    Box::new(Constant("2".to_string())),
                    Some(Mul),
                    Some(Box::new(Constant("3".to_string())))
                )),
                Some(Div),
                Some(Box::new(Constant("4".to_string()))),
            ),)),
        ),
    )
}
