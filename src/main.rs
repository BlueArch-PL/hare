mod parser;
mod tests;

use clap::Command;
use parser::*;
use std::{fmt::format, io::BufRead};

fn print_debug(ast: Vec<AstNode>) {
    parser::utils::debug(format!("AST: {:#?}", ast));
    for node in ast {
        parser::utils::debug(format!("As Code: {:#?}", node.as_code()));
    }
}

fn main() {
    pretty_env_logger::init();
    let matches = Command::new("hare")
        .version("0.0.1")
        .about("Hare is the official compiler of Blue Arch Programming Language")
        .author("XYCode <xycode-xyc@outlook.com>")
        .arg(clap::arg!(-i --input <INPUT> "Input file").required_unless_present("code"))
        .arg(clap::arg!(-c --code <CODE> "Code to be compiled").required_unless_present("input"))
        .get_matches();

    if let Some(code) = matches.get_one::<String>("code") {
        let ast = parse(code).expect("Failed to parse input to AST!");
        print_debug(ast);
    }

    if let Some(file) = matches.get_one::<String>("input") {
        let code = std::fs::read_to_string(file).expect("Failed to read input file!");
        let ast = parse(code.as_str()).expect("Failed to parse input to AST!");
        print_debug(ast);
    }
}
