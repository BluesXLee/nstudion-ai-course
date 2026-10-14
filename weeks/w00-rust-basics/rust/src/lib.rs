//! W0：用一個小型 metrics 告警程式練習所有權、借用、`Result` 與 trait。
//!
//! 資料流：`"cpu=93.5"` 文字 → [`metric::parse_line`] → 數值視窗 → [`stats`] → [`severity::Rule`] → [`severity::Severity`]。
//! 這就是 W16 決策服務的縮小版：只是把「規則」換成模型。
//!
//! # 關鍵問題：為什麼借用規則能阻止資料競爭？
//!
//! 資料競爭需要三個條件同時成立：兩條執行緒存取同一塊記憶體、至少一條在寫、沒有同步。
//! 借用規則「同一時間只能有一個 `&mut`，或任意多個 `&`」直接排除了「一邊寫一邊讀」。
//! 下面這段程式無法編譯（`cargo test --doc` 會驗證它確實編譯失敗）：
//!
//! ```compile_fail
//! let mut window = vec![1.0, 2.0];
//! let first = &window[0];      // 不可變借用還活著
//! window.push(3.0);            // 錯誤：同時要可變借用；push 可能重新配置記憶體，first 會變成懸空指標
//! println!("{first}");
//! ```
//!
//! 跨執行緒也是同一條規則：
//!
//! ```compile_fail
//! let mut total = 0.0;
//! std::thread::scope(|s| {
//!     s.spawn(|| total += 1.0);  // 第一個可變借用
//!     s.spawn(|| total += 2.0);  // 錯誤：第二個可變借用
//! });
//! ```

pub mod metric;
pub mod severity;
pub mod stats;
