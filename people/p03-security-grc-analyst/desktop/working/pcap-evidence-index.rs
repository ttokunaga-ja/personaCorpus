/// Index only metadata supplied by an analyst; this does not read packet captures.
use std::collections::BTreeMap;

fn main() {
    let observations = [("capture-001", "DNS", 14_u32), ("capture-002", "TLS", 9_u32), ("capture-003", "DNS", 4_u32)];
    let mut counts: BTreeMap<&str, u32> = BTreeMap::new();
    for (_, protocol, records) in observations { *counts.entry(protocol).or_default() += records; }
    println!("evidence set 184 protocol_totals={counts:?}");
}
