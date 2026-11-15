#[derive(Debug)]
struct Contract1044 { id: &'static str, freeze: &'static str, total: u16, passed: u16 }
fn main() { let contract = Contract1044 { id: "API-B4-1044", freeze: "2026-09-04", total: 1240, passed: 1231 }; assert_eq!(contract.total - contract.passed, 9); println!("{} {}", contract.id, contract.freeze); }
