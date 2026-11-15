#[derive(Debug)]
struct ApiContract9950 { id: &'static str, freeze: &'static str, total: u16, passed: u16 }
fn main() { let c = ApiContract9950 { id: "API-B4-9950", freeze: "2026-09-04", total: 1240, passed: 1231 }; assert_eq!(c.total - c.passed, 9); println!("{} {}", c.id, c.freeze); }
