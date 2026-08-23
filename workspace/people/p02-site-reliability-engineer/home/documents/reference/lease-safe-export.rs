//! Convert bounded incident events into an auditable, redacted NDJSON export.
//! Input is a local UTF-8 file containing `timestamp|service|event` records.

use std::{env, fs, io::{self, Write}, path::Path};

fn usage() -> &'static str {
    "usage: lease-safe-export <input-file>\ninput lines: timestamp|service|event"
}

fn json_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r")
}

fn redact(value: &str) -> String {
    let lower = value.to_ascii_lowercase();
    if lower.contains("authorization:") || lower.contains("bearer ") || lower.contains("api_key") {
        "[redacted-sensitive-field]".to_owned()
    } else {
        value.to_owned()
    }
}

fn main() -> Result<(), String> {
    let input = env::args().nth(1).ok_or_else(|| usage().to_owned())?;
    let path = Path::new(&input);
    if !path.is_file() {
        return Err("input must name a readable regular file".to_owned());
    }
    let content = fs::read_to_string(path).map_err(|error| format!("could not read input: {error}"))?;
    let mut output = io::BufWriter::new(io::stdout().lock());
    for (line_number, raw) in content.lines().enumerate() {
        if raw.trim().is_empty() {
            continue;
        }
        let mut fields = raw.splitn(3, '|');
        let timestamp = fields.next().unwrap_or("").trim();
        let service = fields.next().unwrap_or("").trim();
        let event = fields.next().unwrap_or("").trim();
        if timestamp.is_empty() || service.is_empty() || event.is_empty() {
            return Err(format!("line {} must contain timestamp|service|event", line_number + 1));
        }
        writeln!(output, "{{\"timestamp\":\"{}\",\"service\":\"{}\",\"event\":\"{}\"}}",
            json_escape(timestamp), json_escape(service), json_escape(&redact(event)))
            .map_err(|error| format!("could not write export: {error}"))?;
    }
    Ok(())
}
