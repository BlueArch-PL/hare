mod parser;
mod tests;

use parser::*;
use std::io::BufRead;

fn main() {
    for line in std::io::stdin().lock().lines() {
        println!(
            "AST: {:#?}",
            parse(line.unwrap().as_str()).expect("Failed to parse input to AST!")
        )

        // for node in ast {
        //     println!("{}", AstNode::format(&node))
        // }
    }
}
