# 在各個平台印出 Rust code 的 log

我使用 [tracing](https://crates.io/crates/tracing) 來印 log，每個平台必須實作各自 [tracing-subscriber](https://crates.io/crates/tracing-subscriber) 的 [layer](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/layer/trait.Layer.html)．

以下是在 [crates.io](https://crates.io/) 上找到了已經實作 layer 的 crates，請參考 [logger-demo](logger-demo) ．

### Android
* [tracing-android](https://crates.io/crates/tracing-android)

### iOS, macOS
* [tracing-oslog](https://crates.io/crates/tracing-oslog)
