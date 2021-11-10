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
        data.category.as_deref().unwrap_or("unknown"),
        data.message.as_deref().unwrap_or("unknown"),
    );

    #[cfg(feature = "telemetry")]
    sentry::add_breadcrumb(data);
}
