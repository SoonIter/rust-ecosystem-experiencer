use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::registry().with(fmt::layer()).init();

    // 调用 `log` 包的 `info!`
    log::info!("Hello world");

    let a = 42;
    // 调用 `tracing` 包的 `info!`
    tracing::info!(a, "Hello from tracing");
}
