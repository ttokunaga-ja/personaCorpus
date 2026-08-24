#[derive(Debug)]
struct ApiContract8696 { id: &'static str, freeze: &'static str, total: u16, passed: u16 }
fn main() { let c = ApiContract8696 { id: "API-B4-8696", freeze: "2026-09-04", total: 1240, passed: 1231 }; assert_eq!(c.total - c.passed, 9); println!("{} {}", c.id, c.freeze); }
