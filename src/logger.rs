pub fn log(data: sentry::Breadcrumb) {
    let level = match data.level {
        sentry::Level::Debug => "DEBUG",
        sentry::Level::Info => "INFO",
        sentry::Level::Warning => "WARNING",
        sentry::Level::Error => "ERROR",
        sentry::Level::Fatal => "FATAL",
    };
    println!(
        "[{}] {}: {}",
        level,
        data.category.clone().unwrap_or("unknown".to_string()),
        data.message.clone().unwrap_or("".to_string())
    );

    #[cfg(feature = "telemetry")]
    sentry::add_breadcrumb(data);
}
