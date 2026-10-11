# 🦀 從 Go 到 Rust：開發者轉型指南

這是一份專為具備 Go 語言經驗的開發者所設計的轉型指南。你將從「透過垃圾回收 (GC) 來換取開發效率」的思維，轉向「透過所有權 (Ownership) 來換取極致性能與安全」的思維。

---

## 1. 核心思維的轉變 (The Mental Shift)

這是最重要的一步。如果你試圖用寫 Go 的方式寫 Rust，你會不斷與編譯器（Borrow Checker）搏斗。

| 特性 | Go (The Managed Way) | Rust (The Ownership Way) |
| :--- | :--- | :--- |
| **內存管理** | **GC (Garbage Collector)**。你只管分配，GC 負責回收。 | **Ownership (所有權)**。編譯器在編譯時就決定了內存何時釋放。 |
| **併發模型** | **Goroutines (CSP)**。輕量級線程，透過 Channel 通訊。 | **Fearless Concurrency**。透過所有權規則防止 Data Race。 |
| **錯誤處理** | `if err != nil`。顯式檢查，風格簡潔。 | `Result<T, E>` 與 `Option<T>`。利用類型系統強制處理。 |
| **抽象/接口** | **Implicit Interfaces**。只要實現了方法，就自動符合接口。 | **Traits**。顯式實現，類型系統更強大且嚴謹。 |
| **性能目標** | 高效率、高開發效率、低延遲。 | 極致性能、零成本抽象 (Zero-cost Abstraction)。 |

---

## 2. 學習路徑建議 (The Roadmap)

建議不要從頭讀《The Rust Programming Language》，而是按照以下階段進行「對比式學習」。

### 第一階段：基礎語法與工具 (The Basics)
* **工具鏈**：熟悉 `rustup` (版本管理), `cargo` (類似 Go 的 `go mod`), `rust-analyzer` (VS Code 插件)。
* **變數與不可變性**：Rust 默認是 `let x = 5;` (不可變)，這與 Go 不同。
* **類型系統**：理解 `struct` 和 `enum`。Rust 的 `enum` 是「和型 (Sum Types)」，比 Go 的 `const` 組合強大得多。

### 第二階段：所有權與借用 (The Hard Part - 核心重點)
這是 Go 開發者最痛苦的階段。你需要理解：
* **Ownership (所有權)**：每個值都有一個主人，主人離開作用域，值就被銷毀。
* **Borrowing (借用)**：
    * `&T` (不可變借用)：可以有多個讀取者。
    * `&mut T` (可變借用)：同一時間只能有一個寫入者，且不能有讀取者。
* **Lifetimes (生命週期)**：編譯器如何確保引用不會指向已釋放的內存。

### 第三階段：錯誤處理與模式匹配 (Error Handling)
* **不再是 `if err != nil`**：學習使用 `match` 和 `if let`。
* **`Option<T>`**：處理「可能為空」的值（取代 Go 的 `nil`）。
* **`Result<T, E>`**：處理「可能出錯」的值。
* **`?` 操作符**：這是 Rust 的「語法糖」，讓你像 Go 一樣簡潔地向上拋出錯誤，但更安全。

###第四階段：Trait 與泛型 (Abstraction)
* **Traits**：理解如何定義行為。這類似於 Go 的 Interface，但更強大。
* **Generics**：學習如何編寫通用的代碼。

###第五階段：併發與 Async (Concurrency)
* **Thread vs Async**：理解 Rust 的線程與 Go 的 Goroutine 的區別。
* **Tokio/async-std**：學習 Rust 的非同步運行時（Rust 沒有內置 runtime，這點與 Go 不同）。

---

## 3. 避坑指南 (Tips for Go Devs)

1. **不要試圖在 Rust 中模擬 GC**：不要試圖用 `Rc<RefCell<T>>` 來解決所有問題。如果你發現自己需要大量使用它們，說明你的數據結構設計可能還在用「對象圖」思維，而不是「所有權」思維。
2. **理解「所有權」是為了「安全」**：在 Go 中，你可能會不小心在多個 Goroutine 中修改同一個指針。在 Rust 中，編譯器會直接攔截這種 Data Race。
3. **學習使用 `Clone`**：在學習初期，如果遇到所有權問題，可以先用 `.clone()` 來解決。這雖然有性能損耗，但能幫你先跑通邏輯，之後再優化。
4. **不要害怕編譯器**：Go 的編譯器是你的助手，Rust 的編譯器是你的「教練」。它在編譯時告訴你的錯誤，是在防止你生產環境的崩潰。

---

## 4. 快速對比範例 (Code Comparison)

### Go: 處理可能為空的對象
```go
func get_name(u *User) string {
    if u == nil {
        return "Unknown"
    }
    return u.Name
}
```

### Rust: 處理可能為空的對象 (使用 Option)
```rust
struct User {
    name: String,
}

fn get_name(u: Option<&User>) -> String {
    // 使用 match 或 if let 進行模式匹配
    match u {
        Some(user) => user.name.clone(),
        None => "Unknown".to_string(),
    }
}
```
