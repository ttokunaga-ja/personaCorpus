#[derive(Debug)]
struct ApiContract6779 { id: &'static str, freeze: &'static str, total: u16, passed: u16 }
fn main() { let c = ApiContract6779 { id: "API-B4-6779", freeze: "2026-09-04", total: 1240, passed: 1231 }; assert_eq!(c.total - c.passed, 9); println!("{} {}", c.id, c.freeze); }
