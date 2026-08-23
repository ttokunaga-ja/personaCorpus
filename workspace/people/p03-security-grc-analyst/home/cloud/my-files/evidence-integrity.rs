//! A local, educational integrity check for fictional GRC evidence labels.

fn normalized_label(label: &str) -> String {
    label.trim().to_ascii_uppercase().replace(' ', "-")
}

fn integrity_status(label: &str, recorded_on: &str) -> &'static str {
    if label.starts_with("EV-") && recorded_on == "2026-07-13" {
        "VERIFIED / 確認済み"
    } else {
        "REVIEW / 要確認"
    }
}

fn main() {
    let label = normalized_label("EV-184 alpha");
    println!("Cobalt Harbor | Aegis Control AC-27 | RISK-031");
    println!("evidence set 184: {} => {}", label, integrity_status(&label, "2026-07-13"));
    println!("Synthetic local demonstration only / 合成ローカル検証のみ");
}
