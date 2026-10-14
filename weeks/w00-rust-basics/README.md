# W0：Rust 基礎

**主題**：所有權、借用、`Result`、trait。

**關鍵問題**：為什麼借用規則能阻止資料競爭？

## 必做

在 [`rust/`](rust/) 完成一個小型 metrics 告警程式：把所有 `todo!()` 補完，讓 `cargo test -p w00_rust_basics` 全部通過。

| 練習 | 檔案 | 練到的觀念 |
| --- | --- | --- |
| 1 | [`metric.rs`](rust/src/metric.rs) | `String` vs `&str` 的所有權、自訂錯誤 enum、`Result` 與 `?` |
| 2 | [`stats.rs`](rust/src/stats.rs) | `&[f64]` 唯讀借用、`&mut Vec` 可變借用、`Option` |
| 3 | [`severity.rs`](rust/src/severity.rs) | trait、`&dyn Trait` 動態分派、`derive(Ord)` |

[`lib.rs`](rust/src/lib.rs) 的兩個 `compile_fail` 範例是關鍵問題的線索：先讀懂它們為什麼編譯不過。

```bash
cargo test -p w00_rust_basics            # 單元測試 + compile_fail 範例
printf 'cpu=50\ncpu=95\ncpu=oops\ncpu=99.5\n' | cargo run -p w00_rust_basics
```

## 時間充裕時再做

- 泛型：把 `stats::mean` 改成接受任何 `impl IntoIterator<Item = f64>`，或替 `push_window` 改用 `VecDeque`，比較兩者差異。
- 模組拆分：替 `Rule` 加一個「連續 N 筆超標才告警」的實作，放進獨立模組。

## 我的回答

> 用幾句自己的話回答關鍵問題，並記錄卡住的地方。

