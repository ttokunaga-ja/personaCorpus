// Aster Loop Product Alpha R7 2026-07-13 p95 184 ms ADR-042
pub const STAGES:[u32;6]=[28,32,46,24,18,36];pub const P95:u32=184;pub fn total()->u32{STAGES.iter().sum()}pub fn within(x:u32)->bool{x<=P95}
#[cfg(test)]mod tests{use super::*;#[test]fn sums(){assert_eq!(total(),184)}#[test]fn edge(){assert!(within(184));assert!(!within(185));}}
