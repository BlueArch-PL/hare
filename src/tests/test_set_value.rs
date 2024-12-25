#[test]
fn test_set_value1() {
    use crate::parser::parse_pairs;
    use crate::parser::BlueArchParser;
    use crate::Rule;
    use pest::Parser;

    let expr = "a = 1";
    let pairs = BlueArchParser::parse(Rule::program, expr);

    assert!(pairs.is_ok());

    let ast = parse_pairs(pairs.unwrap());
    assert!(ast.is_ok());
    let ast = ast.unwrap();

    assert!(ast.len() == 1);
    assert_eq!(ast[0].as_code(), "a = 1;");
}

#[test]
fn test_set_value2() {
    use crate::parser::parse_pairs;
    use crate::parser::BlueArchParser;
    use crate::Rule;
    use pest::Parser;

    let expr = "a = 1 + 2";
    let pairs = BlueArchParser::parse(Rule::program, expr);
    assert!(pairs.is_ok());
    let ast = parse_pairs(pairs.unwrap());
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    assert!(ast.len() == 1);
    assert_eq!(ast[0].as_code(), "a = (1 + 2);");
}

#[test]
fn test_set_value3() {
    use crate::parser::parse_pairs;
    use crate::parser::BlueArchParser;
    use crate::Rule;
    use pest::Parser;

    let expr = "a = 1 + 2 * 3";
    let pairs = BlueArchParser::parse(Rule::program, expr);
    assert!(pairs.is_ok());
    let ast = parse_pairs(pairs.unwrap());
    assert!(ast.is_ok());
    let ast = ast.unwrap();
    assert!(ast.len() == 1);
    assert_eq!(ast[0].as_code(), "a = (1 + (2 * 3));");
}
