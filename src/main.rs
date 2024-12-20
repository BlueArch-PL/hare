mod parser;
mod tests;

use parser::*;
use std::io::BufRead;

fn main() {
    for line in std::io::stdin().lock().lines() {
        println!("AST: {:#?}", parse(line.unwrap().as_str()).unwrap())

        // for node in ast {
        //     println!("{}", AstNode::format(&node))
        // }
    }
}
