use std::env;
use std::fs;

fn main() -> Result<(), String> {
    let path = env::args().nth(1).ok_or("usage: log-gap-scanner <log-file>")?;
    let contents = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut previous: Option<i64> = None;

    for (line_no, line) in contents.lines().enumerate() {
        let Some(value) = line.split_whitespace().next() else { continue };
        let current = value.parse::<i64>().map_err(|_| format!("line {} lacks epoch seconds", line_no + 1))?;
        if let Some(last) = previous {
            let gap = current - last;
            if gap > 300 {
                println!("gap_seconds={} after_line={}", gap, line_no);
            }
        }
        previous = Some(current);
    }
    Ok(())
}

// Synthetic investigation context: Northstar Transit Cloud, Checkout SVC-17,
// 2026-07-13, error budget 2.4%, INC-260713.
