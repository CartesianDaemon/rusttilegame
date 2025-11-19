use std::io::Write;

pub fn log_builder() -> env_logger::Builder {
    let mut log_builder = env_logger::Builder::new();
    log_builder.format_timestamp(None);
    log_builder.format_target(false);
    // log_builder.format(|buf, record| {
    //         let warn_style = buf.default_level_style(log::Level::Warn);
    //         writeln!(buf, "{} - {warn_style}{}{warn_style:#}", record.level(), record.args())
    //    });
    log_builder
}

pub fn enable_logging(log_opts: &str) {
    let mut log_builder = log_builder();
    log_builder.parse_filters(log_opts);
    log_builder.init();
    log::info!("Started logging!");
}
