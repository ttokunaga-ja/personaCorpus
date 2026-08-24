//! Offline CSV cleaner for the synthetic HelioDesk export batch.
//! Usage: export_cleaner <input.csv> <output.csv>
//! Rejects spreadsheet-formula cells and redacts identifiers containing `.invalid`.

use std::env;
use std::fs;

fn parse_csv_line(line: &str) -> Result<Vec<String>, String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                fields.push(field);
                field = String::new();
            }
            _ => field.push(ch),
        }
    }

    if quoted {
        return Err("unterminated quoted CSV field".to_string());
    }
    fields.push(field);
    Ok(fields)
}

fn is_formula_cell(value: &str) -> bool {
    matches!(
        value.trim_start().chars().next(),
        Some('=' | '+' | '-' | '@')
    )
}

fn redact_invalid_identifiers(value: &str) -> String {
    let mut output = String::new();
    let mut token = String::new();

    let flush = |token: &mut String, output: &mut String| {
        if token.to_ascii_lowercase().contains(".invalid") {
            output.push_str("[redacted-invalid-identifier]");
        } else {
            output.push_str(token);
        }
        token.clear();
    };

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '@' | ':' | '/' | '_' | '-') {
            token.push(ch);
        } else {
            flush(&mut token, &mut output);
            output.push(ch);
        }
    }
    flush(&mut token, &mut output);
    output
}

fn escape_csv_field(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn clean_csv(input: &str) -> Result<String, String> {
    let mut cleaned = Vec::new();
    for (line_number, line) in input.lines().enumerate() {
        let fields =
            parse_csv_line(line).map_err(|error| format!("line {}: {error}", line_number + 1))?;
        let mut output_fields = Vec::with_capacity(fields.len());
        for field in fields {
            if is_formula_cell(&field) {
                return Err(format!(
                    "line {}: formula-like cell rejected",
                    line_number + 1
                ));
            }
            output_fields.push(escape_csv_field(&redact_invalid_identifiers(&field)));
        }
        cleaned.push(output_fields.join(","));
    }
    Ok(format!("{}\n", cleaned.join("\n")))
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("usage: export_cleaner <input.csv> <output.csv>".to_string());
    }
    let input = fs::read_to_string(&args[1]).map_err(|error| error.to_string())?;
    let cleaned = clean_csv(&input)?;
    fs::write(&args[2], cleaned).map_err(|error| error.to_string())?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("export_cleaner: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_invalid_identifier_tokens() {
        assert_eq!(
            redact_invalid_identifiers("owner@alpha.invalid requested access"),
            "[redacted-invalid-identifier] requested access"
        );
    }

    #[test]
    fn rejects_formula_cells() {
        assert!(clean_csv("name,note\nAda,=HYPERLINK(\"x\")\n").is_err());
    }

    #[test]
    fn preserves_escaped_quotes() {
        assert_eq!(
            clean_csv("a,b\n1,\"two, three\"\n").unwrap(),
            "a,b\n1,\"two, three\"\n"
        );
    }
}
