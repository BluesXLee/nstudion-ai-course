# 🦀 Rust 快速上手指南 (針對 ML 實作)

這份文件旨在協助熟悉 Python 的開發者快速掌握 Rust，方便在 `candle` 框架中處理 ML 模型。

## 1. 核心工具鏈 (Tooling)

Rust 的開發流程非常成熟：
- **`rustup`**：版本管理工具（類似 `pyenv` 或 `conda`）。
- **`cargo`**：包管理與建構工具（類似 `pip` + `poetry` + `cmake`）。
  - `cargo new`：建立新專案。
  - `cargo build`：編譯專案。
  - `cargo run`：編譯並執行。
  - `cargo test`：執行測試。
- **`Cargo.toml`**：專案設定與依賴定義（類似 `requirements.txt` 或 `pyproject.toml`）。

---

## 2. 記憶體管理：所有權 (Ownership) 與借用 (Borrowing)

這是 Rust 與 Python 最大的概念差異。Rust 不使用垃圾回收 (GC)，而是透過編譯時的 **所有權 (Ownership)** 系統來處理記憶體釋放。

### 2.1 所有權 (Ownership)

變數擁有它的值。例如：
```rust
let s2 = s1; // s1 的所有權被「移動 (Move)」到了 s2
```
在 Python 中，一個變數可以有多個引用指向同一個物件，但在 Rust 中，同一個值一次只能有一個所有者，以防止記憶體被重複釋放。

### 2.2 借用 (Borrowing)

如果不想移動所有權，可以使用引用：
- **不可變借用 (`&`)**：可以讀取但不能修改。
- **可變借用 (`&mut`)**：同一時間只能有一個寫入者，且不能與讀取者並存。

---

## 3. 型別系統與資料結構

- **結構體 (Structs)**：用於定義模型參數或訓練設定。
- **列舉 (Enums) 與模式匹配 (Pattern Matching)**：Rust 的 Enum 可以攜帶資料。這在處理不同硬件（如 CPU/CUDA）時非常有用。

---

## 4. 錯誤處理 (Error Handling)

Rust 使用 `Result<T, E>`，而不是 `try/except`。例如：
```rust
weights = read_file("model.bin")?;
```
中的 `?` 表示：「如果成功就取出值；如果是錯誤就直接將錯誤 return 給呼叫者」。這讓程式碼更清爽，不需要額外寫 `try/except`。

---

## 5. 針對 `candle` 與 `rust_matrix` 的關鍵

- **`usize`**：維度、索引與圈覽元件數量。
- **`anyhow`**：專案用來簡化回傳的錯誤定義。
- **避免過度使用 `.clone()`**：在 ML 運算中，如果編譯器抱怨所有權移動，可考慮使用 `&` 引用，而不是 `.clone()`。

---

## 快速對照表

| 概念 | Python | Rust |
|-------|--------|------|
| 變數定義 | `x = 1` | `let x = 1;`（不可變） / `let mut x = 1;`（可變） |
| 列表/陣列 | `list = [1, 2]` | `let v = vec![1, 2];` |
| 字典 | `dict = {"a": 1}` | `HashMap::new()` |
| 函數定義 | `def func(a):` | `fn func(a: i3s) -> i32 { ... }` |
| 錯誤處理 | `try...except` | `Result<T, E>` + `match` 或 `?` |
| 類別/物件 | `class User:` | `struct User { ... }` + `impl User { ... }` |

---

## 建議練習路徑

1. **熟悉 Cargo**：嘗試在 `rust_matrix` 中新增一個 `bin` 檔案並執行。
2. **實作簡單矩陣**：嘗試用 `Vec<f32>` 實作一個簡單的矩陣乘法，感受 `&` 與 `mut` 的差異。
3. **對接 Candle**：嘗試建立一個 `candle_core::Tensor` 並進行簡單的加法運算。

---

## 附加資源

- [The Rust Programming Language (rust-lang.org)](https://doc.rust-lang.org/stable/book/)
- [Candle Documentation](https://candle.dev/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)

---

## 常見陷阱

1. **所有權移動**：Rust 的嚴格所有權系統初學時可能會感到困惑，但這是記憶體安全的保證。
2. **借用檢查器**：借用檢查器可能看起來過於嚴格，但它在編譯時期防止了資料競爭。
3. **型別系統**：Rust 的型別系統非常嚴格，可能需要比 Python 更多的型別註解。
4. **錯誤處理**：Rust 的錯誤處理比 Python 更明確，可能導致程式碼更冗長。

---

s
## 結語

Rust 是一個強大的語言，特別適合 ML 模型實作。雖然從 Python 轉換可能會有挑戰，但它在效能、記憶安全與明確的錯誤處理上的優勢，使得這個努力非常值得。
