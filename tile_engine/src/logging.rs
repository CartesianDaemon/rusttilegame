pub fn log_builder() -> env_logger::Builder {
    let mut log_builder = env_logger::Builder::new();
    log_builder.format_timestamp(None);
    log_builder.format_target(false);
    log_builder
}

pub fn enable_logging(log_opts: &str) {
    let mut log_builder = log_builder();
    log_builder.parse_filters(log_opts);
    log_builder.init();
    log::info!("Started logging!");
}
