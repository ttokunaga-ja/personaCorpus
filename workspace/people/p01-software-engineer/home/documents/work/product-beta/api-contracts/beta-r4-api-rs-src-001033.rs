#[derive(Debug)]
struct Contract1033 { id: &'static str, freeze: &'static str, total: u16, passed: u16 }
fn main() { let contract = Contract1033 { id: "API-B4-1033", freeze: "2026-09-04", total: 1240, passed: 1231 }; assert_eq!(contract.total - contract.passed, 9); println!("{} {}", contract.id, contract.freeze); }
