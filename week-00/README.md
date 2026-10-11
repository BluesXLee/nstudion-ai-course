# Week 0：Rust 基礎——所有權、借用、`Result` 與 trait

本週目標：寫出一個可測試的小型 Rust 程式，含錯誤處理與單元測試。後面每週的 Candle 練習都會用到所有權、借用與 `Result`，這週先把這些語法跟編譯器錯誤訊息摸熟。

## 必做產出

### 1. 包裹狀態機（擴充現有程式）
- 程式：[`rust_course/src/main.rs`](rust_course/src/main.rs)（套件名稱 `rust_course`）
- 執行：
  ```bash
  source "$HOME/.cargo/env"
  cargo run -p rust_course
  cargo test -p rust_course
  ```
- 現有程式已經示範了 `enum`、`struct`、`impl`、`&mut self` 與 `match`。在它之上完成：
  1. **錯誤處理**：把 `update_status` 改成回傳 `Result<(), StatusError>`，拒絕不合法的狀態轉換（例如 `Delivered` 之後不能再變回 `Shipped`、`Ordered` 不能直接跳到 `Delivered`）。自訂 `StatusError` enum，並在 `main` 裡用 `match` 或 `?` 處理錯誤。
  2. **trait**：為 `Package` 實作 `std::fmt::Display`，用它取代 `print_status` 的手動格式化。
  3. **單元測試**：在 `#[cfg(test)] mod tests` 裡至少寫三個測試——一個合法轉換、一個不合法轉換回傳 `Err`、一個 `Display` 的輸出內容。

### 2. 觀察借用規則如何阻止資料競爭
- 寫一段同時持有兩個 `&mut` 的程式，讀懂編譯器的錯誤訊息。
- 用 `std::thread::spawn` 讓兩個執行緒修改同一個 `Vec`，觀察為什麼編譯不過；再改用 `Arc<Mutex<Vec<_>>>` 讓它通過。
- 這段可以寫在測試裡或另開一個小檔案，重點是看懂錯誤訊息，不需要保留失敗的版本。

## 時間充裕時再做

- **泛型**：寫一個 `fn latest<T: Clone>(history: &[T]) -> Option<T>`，記錄並回傳最後一個狀態。
- **模組拆分**：把 `PackageStatus`、`Package`、`StatusError` 拆到 `src/package.rs`，在 `main.rs` 用 `mod package;` 引入。
- 參考閱讀：[`Rust_for_Go_Devs.md`](../docs/Rust_for_Go_Devs.md)，以及 [`rust_mastery/src/`](../rust_mastery/src/) 的 ch06（struct）、ch07（enum）、ch08（錯誤處理）、ch09（trait）。

## 要能答

> 為什麼借用規則能阻止資料競爭？

## 我的解釋

<!-- 完成練習後，用不依賴公式的白話寫在這裡。 -->
