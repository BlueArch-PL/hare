mod parser;
mod tests;

use parser::*;
use std::io::BufRead;

fn main() {
    pretty_env_logger::init();

    for line in std::io::stdin().lock().lines() {
        let ast = parse(line.unwrap().as_str()).expect("Failed to parse input to AST!");

        parser::utils::debug(format!("AST: {:#?}", ast));

        for node in ast {
            parser::utils::debug(format!("As Code: {:#?}", node.as_code()));
        }

        // for node in ast {
            // println!("{}", AstNode::format(&node))
        // }
    }
}
