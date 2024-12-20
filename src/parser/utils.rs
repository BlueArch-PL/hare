use pest::iterators::{Pair, Pairs};
use crate::grammar::Rule;

pub fn print_pair(pair: &Pair<Rule>, level: Option<u8>) {
    let pair = pair.clone();

    println!(
        "{} [{:?}] {:?}",
        " ".repeat(level.unwrap() as usize),
        pair.as_rule(),
        pair.as_str()
    );

    let inner = pair.into_inner();
    if inner.len() > 0 {
        print_pairs(&inner, Some(level.unwrap() + 1));
    }
}

pub fn print_pairs(pairs: &Pairs<Rule>, level: Option<u8>) {
    let pairs = pairs.clone();
    for pair in pairs {
        print_pair(&pair, level);
    }
}