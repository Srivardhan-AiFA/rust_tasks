use std::fs::OpenOptions;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_tracing() {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .expect("Failed to open log file");

    let stdout = std::io::stdout;

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(stdout.and(file))
        .init();
}
