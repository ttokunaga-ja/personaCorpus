mod autopilot;
mod batch;
mod check;
mod manifest;
mod media;
mod ops;
mod pack;
mod rename;
fn base() -> std::path::PathBuf {
    std::env::var_os("CORPUS_BASE")
        .map(Into::into)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .into()
        })
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let code = match args.first().map(String::as_str) {
        Some("--help" | "-h") => {
            println!(
                "usage: corpus <check|batch|manifest|pack|render|office-pdf|tex|scan|stamp|recalc|autopilot|rename|job|alert|watchd> [arguments]"
            );
            0
        }
        Some("manifest") => manifest::run(&args[1..]),
        Some("pack") => pack::run(&args[1..]),
        Some("check") => check::run(&args[1..]),
        Some("autopilot") => autopilot::run(&args[1..]),
        Some("batch") => batch::run(&args[1..]),
        Some("rename") => rename::run(&args[1..]),
        Some(c @ ("render" | "office-pdf" | "tex" | "scan" | "stamp" | "recalc")) => {
            media::run(c, &args[1..])
        }
        Some(c @ ("job" | "alert" | "watchd" | "codex-job" | "agent-alert" | "agent-watchd")) => {
            ops::run(c, &args[1..])
        }
        _ => {
            eprintln!(
                "usage: corpus <check|batch|manifest|pack|render|office-pdf|tex|scan|stamp|recalc|autopilot|rename|job|alert|watchd> [arguments]"
            );
            2
        }
    };
    std::process::exit(code)
}
