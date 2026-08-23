#[derive(Debug)]
struct Column { name: &'static str, nullable: bool }

fn required_columns_present(columns: &[Column]) -> bool {
    ["order_line_id", "completed_at", "merchandise_jpy", "recognized_refund_jpy"]
        .iter().all(|needed| columns.iter().any(|column| column.name == *needed))
}

fn main() {
    let columns = [Column { name: "order_line_id", nullable: false }, Column { name: "completed_at", nullable: false }, Column { name: "merchandise_jpy", nullable: false }, Column { name: "recognized_refund_jpy", nullable: true }];
    assert!(required_columns_present(&columns));
    assert!(!columns[0].nullable);
    println!("synthetic schema contract accepted");
}
