use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

// 初始化 tracing-subscriber，只執行一次即可
pub fn init() {
    #[cfg(target_os = "android")]
    {
        let layer = tracing_android::layer("katana").unwrap();
        tracing_subscriber::registry().with(layer).init();
    }
    
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    {
        let layer = tracing_oslog::OsLogger::new("com.teon.katana", "main");
        tracing_subscriber::registry().with(layer).init();
    }

    // Linux 和 Windows 使用 tracing_subscriber 內建 fmt layer，不需要額外的 crate 
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    {
        let layer = tracing_subscriber::fmt::Layer::default().pretty();
        tracing_subscriber::registry().with(layer).init();
    }

    info!("logger initialized");
}

// 顯示 log
pub fn show_log() {
    trace!("show log from Rust!!!");
    debug!("show log from Rust!!!");
    info!("show log from Rust!!!");
    warn!("show log from Rust!!!");
    error!("show log from Rust!!!");
}
