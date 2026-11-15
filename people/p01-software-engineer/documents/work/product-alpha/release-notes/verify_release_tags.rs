use std::collections::BTreeSet;

pub fn verify_release_tags(tags: &[(&str, &str)], expected_date: &str) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    for (tag, ancestry_date) in tags {
        let semver = tag.strip_prefix('v').ok_or_else(|| format!("missing v prefix: {tag}"))?;
        let parts: Vec<_> = semver.split('.').collect();
        if parts.len() != 3 || parts.iter().any(|part| part.parse::<u64>().is_err()) {
            return Err(format!("invalid semver: {tag}"));
        }
        if !seen.insert(*tag) { return Err(format!("duplicate tag: {tag}")); }
        if *ancestry_date > expected_date { return Err(format!("tag after release date: {tag}")); }
    }
    Ok(())
}
