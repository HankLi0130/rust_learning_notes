# Basic

## 建立專案

* 用 command line 建立 Rust 專案

```bash
# 建立 hello_rust 專案（包含 main）
cargo new hello_rust 

# 建立 hello_rust 函式庫
cargo new hello_rust --lib 

# 建立 hello_rust 函式庫，不設定版本控制
cargo new hello_rust --lib --vcs=none
```

* 先建立資料夾再建立 Rust 專案

```bash
# 建立 hello_rust 資料夾
mkdir hello_rust

# 進入 hello_rust 專案
cd hello_rust

# 建立 Rust 專案
cargo init --lib --vcs=none
```

## 建立 submodule

### 根據範例，我想建立一個 crypto 的 submodule，應該怎麼做？

![submodule example](pic/submodule_1.png)

1. 首先，建立資料夾 `crypto`
2. 然後進入 `crypto` 資料夾，建立 `mod.rs`
3. 之後再根據需求增加其他的 module，例如 `aes_gcm.rs`

## 什麼是 `default-features = false` ?

在 Rust 的 `Cargo.toml` 中的 `dependencies`，`default-features = false` 的設定表示停用該依賴項的預設功能集（default features）。

```toml
reqwest = { version = "0.12.12", features = ["blocking", "json"], default-features = false }
```

這意味著：

不會啟用 reqwest 的預設功能集，而只啟用你明確指定的功能
在這個案例中，你明確啟用了 "blocking" 和 "json" 兩個功能

這種做法的好處是：

* 減少編譯時間和最終二進制檔案的大小
* 只引入你實際需要的功能，避免不必要的依賴
* 更精確地控制你的應用程序使用哪些功能

reqwest 的預設功能集包括 TLS、cookies 支援等內容。通過設定 default-features = false 並明確選擇 "blocking" 和 "json"，你的應用只會包含阻塞式 HTTP 請求和 JSON 處理的功能，而不包含其他預設功能。

