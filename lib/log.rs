use tracing::Level;

pub fn log(is_debug: bool) {
    let filter = if is_debug { Level::TRACE } else { Level::INFO };

    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(filter)
        .init();
}
