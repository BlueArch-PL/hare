use super::grammar::Rule;
use log::{debug, error, info, trace, warn};
use pest::iterators::{Pair, Pairs};

pub fn print_pair(pair: &Pair<Rule>, level: Option<u8>) {
    let pair = pair.clone();

    debug(format!(
        "{} [{:?}] {:?}",
        " ".repeat(level.unwrap() as usize),
        pair.as_rule(),
        pair.as_str()
    ));

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

#[allow(dead_code)]
#[inline]
pub fn debug(message: String) {
    debug!(target:"parser", "{}", message);
}

#[allow(dead_code)]
#[inline]
pub fn error(message: String) {
    error!(target:"parser", "{}", message);
}

#[allow(dead_code)]
#[inline]
pub fn info(message: String) {
    info!(target:"parser", "{}", message);
}

#[allow(dead_code)]
#[inline]
pub fn trace(message: String) {
    trace!(target:"parser", "{}", message);
}

#[allow(dead_code)]
#[inline]
pub fn warn(message: String) {
    warn!(target:"parser", "{}", message);
}
