//! Standalone validator for the fixed HelioDesk four-hour SLA window.
use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).expect("usage: validate_sla_windows <open-ticket-json>");
    let input = fs::read_to_string(path).expect("read export");
    for required in ["\"ticket_count\": 17", "\"sla_target_hours\": 4", "2026-08-04T09:00:00Z"] {
        assert!(input.contains(required), "missing required export declaration: {required}");
    }
    let tickets = input.matches("\"ticket_id\"").count();
    assert_eq!(tickets, 17, "expected exactly 17 ticket records");
    let within = input.matches("\"sla_state\":\"within\"").count();
    let at_risk = input.matches("\"sla_state\":\"at-risk\"").count();
    let breached = input.matches("\"sla_state\":\"breached\"").count();
    assert_eq!((within, at_risk, breached), (11, 4, 2), "SLA state split mismatch");
    println!("SLA declarations valid: 17 tickets, four-hour target, 11/4/2 state split");
}
