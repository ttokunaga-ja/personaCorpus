// Synthetic Study Alpha cohort A-24 summary helper.
fn median(mut values: Vec<f64>) -> f64 { values.sort_by(|a,b| a.partial_cmp(b).unwrap()); values[values.len()/2] }
fn main() { let values=vec![6.7,6.8,6.9,6.8,6.8]; println!("Study Alpha synthetic median: {:.1} ng/mL", median(values)); }
