#[allow(dead_code)]
fn create_constant(value: &str) -> crate::parser::AstNode {
    use crate::parser::*;

    AstNode::Constant(value.to_string())
}

#[test]
fn test_expr1() {
    use crate::parser::*;

    use pest::Parser;

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
            Box::new(create_constant("1")),
            Some(BinaryOp::Add),
            Some(Box::new(create_constant("2"))),
        )
    );
}

#[test]
fn test_expr2() {
    use crate::parser::*;

    use pest::Parser;

    let expr = "1 - 2 * 3;";
    let pairs = BlueArchParser::parse(Rule::program, expr);

    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0],
        AstNode::Expr(
            Box::new(create_constant("1")),
            Some(BinaryOp::Sub),
            Some(Box::new(AstNode::Expr(
                Box::new(create_constant("2")),
                Some(BinaryOp::Mul),
                Some(Box::new(create_constant("3"))),
            ))),
        )
    );
}

#[test]
fn test_expr3() {
    use crate::parser::*;

    use pest::Parser;

    let expr = "1 + 2 * 3 - 4 / 5;";
    let pairs = BlueArchParser::parse(Rule::program, expr);

    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0],
        AstNode::Expr(
            Box::new(AstNode::Expr(
                Box::new(create_constant("1")),
                Some(BinaryOp::Add),
                Some(Box::new(AstNode::Expr(
                    Box::new(create_constant("2")),
                    Some(BinaryOp::Mul),
                    Some(Box::new(create_constant("3"))),
                ))),
            )),
            Some(BinaryOp::Sub),
            Some(Box::new(AstNode::Expr(
                Box::new(create_constant("4")),
                Some(BinaryOp::Div),
                Some(Box::new(create_constant("5"))),
            ))),
        )
    );
}

#[test]
fn test_expr4() {
    use crate::parser::*;

    use pest::Parser;

    let expr = "1 + 2 * 3 - 4 / 5 * 6;";
    let pairs = BlueArchParser::parse(Rule::program, expr);

    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0],
        AstNode::Expr(
            Box::new(AstNode::Expr(
                Box::new(create_constant("1")),
                Some(BinaryOp::Add),
                Some(Box::new(AstNode::Expr(
                    Box::new(create_constant("2")),
                    Some(BinaryOp::Mul),
                    Some(Box::new(create_constant("3"))),
                ))),
            )),
            Some(BinaryOp::Sub),
            Some(Box::new(AstNode::Expr(
                Box::new(AstNode::Expr(
                    Box::new(create_constant("4")),
                    Some(BinaryOp::Div),
                    Some(Box::new(create_constant("5"))),
                )),
                Some(BinaryOp::Mul),
                Some(Box::new(create_constant("6"))),
            ))),
        )
    );
}

#[test]
fn test_expr5() {
    use crate::parser::*;

    use pest::Parser;

    let expr = "1 + 2 * 3 - 4 / 5 * 6 + 7;";
    let pairs = BlueArchParser::parse(Rule::program, expr);

    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0],
        AstNode::Expr(
            Box::new(AstNode::Expr(
                Box::new(AstNode::Expr(
                    Box::new(create_constant("1")),
                    Some(BinaryOp::Add),
                    Some(Box::new(AstNode::Expr(
                        Box::new(create_constant("2")),
                        Some(BinaryOp::Mul),
                        Some(Box::new(create_constant("3"))),
                    ))),
                )),
                Some(BinaryOp::Sub),
                Some(Box::new(AstNode::Expr(
                    Box::new(AstNode::Expr(
                        Box::new(create_constant("4")),
                        Some(BinaryOp::Div),
                        Some(Box::new(create_constant("5"))),
                    )),
                    Some(BinaryOp::Mul),
                    Some(Box::new(create_constant("6"))),
                ))),
            )),
            Some(BinaryOp::Add),
            Some(Box::new(create_constant("7"))),
        )
    );
}

#[test]
fn test_expr6() {
    use crate::parser::*;

    use pest::Parser;

    let expr = "1 + 2 * 3 - 4 / 5 * 6 + 7 - 8 / 9 * 10;";
    let pairs = BlueArchParser::parse(Rule::program, expr);

    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert_eq!(ast.len(), 1);
    assert_eq!(
        ast[0].as_code(),
        "((((1 + (2 * 3)) - ((4 / 5) * 6)) + 7) - ((8 / 9) * 10))"
    );
}
