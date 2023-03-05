use std::fmt::Display;

use tracing_appender::non_blocking::WorkerGuard;

#[derive(Debug, Clone)]
pub struct Coordinates<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug)]
pub struct Dimension<T> {
    pub width: T,
    pub height: T,
}

impl Display for Dimension<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl From<Dimension<u32>> for String {
    fn from(value: Dimension<u32>) -> Self {
        format!("{}x{}", value.width, value.height)
    }
}

pub fn init_tracing() -> WorkerGuard {
    use tracing_subscriber::fmt::format::FmtSpan;
    let exe_path = std::env::current_exe().unwrap();
    let log_dir = exe_path.parent().unwrap().clone();
    let file_appender = tracing_appender::rolling::never(log_dir, "walksnail-osd-overay-tool.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::CLOSE)
        .init();
    guard
}
