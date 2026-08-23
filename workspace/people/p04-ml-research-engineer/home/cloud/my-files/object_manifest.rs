//! A small, dependency-free manifest model for synthetic evaluation objects.

#[derive(Debug, Clone, PartialEq, Eq)]
struct Object<'a> {
    key: &'a str,
    checksum: &'a str,
    bytes: u64,
}

fn six_hex(value: &str) -> bool {
    value.len() == 6 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn validate(objects: &[Object<'_>]) -> Result<u64, String> {
    let mut total = 0_u64;
    for object in objects {
        if !object.key.starts_with("eval/") || object.key.ends_with('/') {
            return Err(format!("invalid object key: {}", object.key));
        }
        if !six_hex(object.checksum) {
            return Err(format!("invalid checksum for {}", object.key));
        }
        total = total.checked_add(object.bytes).ok_or("byte total overflow")?;
    }
    Ok(total)
}

fn main() {
    let manifest = [
        Object { key: "eval/R184/metrics.json", checksum: "9ac27e", bytes: 2_048 },
        Object { key: "eval/R184/predictions.jsonl", checksum: "bc8154", bytes: 184_320 },
        Object { key: "eval/R184/thresholds.csv", checksum: "4f02ad", bytes: 1_024 },
    ];
    match validate(&manifest) {
        Ok(bytes) => println!("Helix Forge Lab manifest: {} objects, {} bytes", manifest.len(), bytes),
        Err(problem) => eprintln!("manifest rejected: {problem}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_reviewable_evaluation_objects() {
        let objects = [Object { key: "eval/R184/metrics.json", checksum: "a1b2c3", bytes: 12 }];
        assert_eq!(validate(&objects), Ok(12));
    }

    #[test]
    fn rejects_non_hex_or_non_evaluation_entries() {
        assert!(validate(&[Object { key: "notes/readme.txt", checksum: "a1b2c3", bytes: 1 }]).is_err());
        assert!(validate(&[Object { key: "eval/R184/x", checksum: "short", bytes: 1 }]).is_err());
    }
}
