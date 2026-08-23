//! Offline normalizer for synthetic GRC export manifests.
//! It removes duplicate, blank logical record names and prints a JSON array.

use std::collections::BTreeSet;
use std::env;

fn escape_json(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn main() {
    let mut unique = BTreeSet::new();
    for raw in env::args().skip(1) {
        let value = raw.split_whitespace().collect::<Vec<_>>().join(" ");
        if !value.is_empty() {
            unique.insert(value);
        }
    }
    let items: Vec<String> = unique.iter().map(|item| format!("\"{}\"", escape_json(item))).collect();
    println!("[{}]", items.join(","));
}
