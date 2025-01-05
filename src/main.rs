mod compiler;
mod parser;
mod tests;

use clap::Command;
use compiler::print_bytecodes;
use parser::*;
fn main() {
    pretty_env_logger::init();
    let matches = Command::new("hare")
        .version("0.0.1")
        .about("Hare is the official compiler of Blue Arch Programming Language")
        .author("XYCode <xycode-xyc@outlook.com>")
        .arg(clap::arg!(-i --input <INPUT> "Input file").required_unless_present("code"))
        .arg(clap::arg!(-c --code <CODE> "Code to be compiled").required_unless_present("input"))
        .get_matches();

    let _ast: AstNode;

    if let Some(code) = matches.get_one::<String>("code") {
        _ast = parse(code).expect("Failed to parse input to AST!");
    } else if let Some(file) = matches.get_one::<String>("input") {
        let code = std::fs::read_to_string(file).expect("Failed to read input file!");
        _ast = parse(code.as_str()).expect("Failed to parse input to AST!");
    } else {
        println!("No input file or code provided!");
        return;
    }

    let codes = _ast.compile().expect("Failed to compile AST!");
    print_bytecodes(&codes);
}
