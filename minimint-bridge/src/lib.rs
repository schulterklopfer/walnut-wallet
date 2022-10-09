mod api;
mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod client;
mod client_manager;
mod database;
mod payments;
mod tests;

pub fn init_tracing() {
    tracing::info!("called init()");
    // Configure logging
    #[cfg(target_os = "android")]
    use tracing_subscriber::{layer::SubscriberExt, prelude::*, Layer};
    #[cfg(target_os = "android")]
    tracing_subscriber::registry()
        .with(
            paranoid_android::layer("com.justinmoon.fluttermint")
                .with_filter(tracing_subscriber::filter::LevelFilter::INFO),
        )
        .try_init()
        .unwrap_or_else(|error| tracing::info!("Error installing logger: {}", error));

    #[cfg(target_os = "ios")]
    use tracing_subscriber::{layer::SubscriberExt, prelude::*, Layer};
    #[cfg(target_os = "ios")]
    tracing_subscriber::registry()
        .with(
            tracing_oslog::OsLogger::new(
                "com.justinmoon.fluttermint",
                "INFO", // I don't know what this does ...
            )
            .with_filter(tracing_subscriber::filter::LevelFilter::INFO),
        )
        .try_init()
        .unwrap_or_else(|error| tracing::info!("Error installing logger: {}", error));

    #[cfg(target_os = "macos")]
    tracing_subscriber::fmt()
        .try_init()
        .unwrap_or_else(|error| tracing::info!("Error installing logger: {}", error));
}
