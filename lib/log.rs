use tracing::Level;

pub fn log(is_debug: bool) {
    let filter = if is_debug { Level::TRACE } else { Level::INFO };
    // 根据debug 模式设置日志格式
    tracing_subscriber::fmt().pretty()
        .without_time()
        .with_file(is_debug)
        .with_line_number(is_debug)
        .with_target(is_debug)
        .with_max_level(filter)
        .init();
}
