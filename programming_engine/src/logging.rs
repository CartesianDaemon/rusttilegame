pub fn log_builder() -> env_logger::Builder {
    let mut log_builder = env_logger::Builder::new();
    log_builder.format_timestamp(None);
    log_builder.format_target(false);
    if false {
        log_builder.format(|buf, record| {
                let warn_style = buf.default_level_style(log::Level::Warn);
                use std::io::Write;
                writeln!(buf, "{} - {warn_style}{}{warn_style:#}", record.level(), record.args())
        });
    }
    log_builder
}

pub fn enable_logging(log_opts: &str) {
    let mut log_builder = log_builder();
    log_builder.parse_filters(log_opts);
    log_builder.init();
    log::info!("Started logging!");
}

static INITIALISE_ONCE: std::sync::Once = std::sync::Once::new();

pub fn initialise_logging_for_tests() {
    INITIALISE_ONCE.call_once(|| {
        crate::infra::log_builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .init();
        log::info!("Initialised logging for tests.");
    });
}
