#[test]
fn test_assign1() {
    use crate::parser::parse_pairs;
    use crate::parser::BlueArchParser;
    use crate::Rule;
    use pest::Parser;

    let expr = "let a = 1";
    let pairs = BlueArchParser::parse(Rule::program, expr);

    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert!(ast.len() == 1);
    assert_eq!(ast[0].as_code(), "let a = 1;");
}

#[test]
fn test_assign2() {
    use crate::parser::parse_pairs;
    use crate::parser::BlueArchParser;
    use crate::Rule;
    use pest::Parser;

    let expr = "let a = 1 + 2";
    let pairs = BlueArchParser::parse(Rule::program, expr);
    assert!(pairs.is_ok());
    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    assert!(ast.len() == 1);
    assert_eq!(ast[0].as_code(), "let a = (1 + 2);");
}

#[test]
fn test_assign3() {
    use crate::parser::parse_pairs;
    use crate::parser::BlueArchParser;
    use crate::Rule;
    use pest::Parser;

    let expr = "let a: int = 1 + 2 + 3";
    let pairs = BlueArchParser::parse(Rule::program, expr);
    assert!(pairs.is_ok());
    let ast = parse_pairs(pairs.unwrap(), Some(0));
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    assert!(ast.len() == 1);
    assert_eq!(ast[0].as_code(), "let a: int = ((1 + 2) + 3);");
}

#[test]
fn test_assign4() {
    use crate::parser::BlueArchParser;
    use crate::Rule;
    use pest::Parser;

    let expr = "a: int = 1";
    let pairs = BlueArchParser::parse(Rule::program, expr);
    assert!(!pairs.is_ok());
}
