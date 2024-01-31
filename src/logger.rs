use tracing::level_filters::LevelFilter;

pub fn setup_logging(verbosity: u8) {
    let tracing_level = match verbosity {
        0 => LevelFilter::OFF,
        1 => LevelFilter::INFO,
        2 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };

    tracing_subscriber::fmt()
        .with_max_level(tracing_level)
        .init();
}
