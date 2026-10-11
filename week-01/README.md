# Week 1：矩陣、Shape、啟動函數與 Softmax

本週目標：每個數字都知道從哪裡來——能推導矩陣乘法的 shape，並理解 softmax 如何把 logits 變成機率、temperature 如何改變分布。Softmax 與 temperature 是主線的起點：最終的決策服務就是輸出一組經過校正的機率（見 W15–W16）。

## 必做產出

### 1. Python（直覺與可視化）
- 檔案：[`matrix_basics.ipynb`](matrix_basics.ipynb)
- 練習：
  1. 不用 NumPy，手寫向量內積與二維矩陣乘法，並印出輸入與輸出的 shape。
  2. 用 NumPy 驗證矩陣乘法結果，並試幾組 broadcasting（例如 `(3, 1) + (1, 4)`），先預測 shape 再執行。
  3. 畫出常見啟動函數（ReLU、sigmoid、tanh）。
  4. 實作穩定版 softmax（先減去最大值），用很大的 logits（例如 `[1000, 1001, 1002]`）比較穩定版與直接計算的差異。
  5. 畫出同一組 logits 在 $T = 0.5, 1.0, 2.0$ 時的 softmax 分布。

### 2. Rust x Candle（張量與 shape）
- 程式：[`rust_course/src/main.rs`](rust_course/src/main.rs)（套件名稱 `rust_matrix`）
- 執行：
  ```bash
  source "$HOME/.cargo/env"
  cargo run -p rust_matrix
  ```
- 練習：建立 Candle `Tensor`，用 `.shape()` 檢查矩陣乘法前後的 shape。

## 時間充裕時再做

- 用 Rust slice（`&[f32]`）與一維連續記憶體（`row * cols + col`）手寫矩陣乘法，觀察 Candle 的 `.stride()`，並驗證與 Candle `matmul` 的數值一致。
- 外積 $u \cdot v^T$ 觀察 Rank-1 矩陣；用 SVD 把一張圖壓成 rank-10 再還原，觀察奇異值衰減。這是 W14 LoRA 的低秩直覺。

## 要能答

> 矩陣乘法的輸入輸出 shape 如何推導？調整 temperature 時，softmax 的分布在數學上發生什麼變化？

延伸題（有做 SVD 才需要答）：

> 在什麼條件下，rank-10 近似可以留住大部分資訊？從奇異值分布可以看出什麼？

## 我的解釋

<!-- 完成練習後，用不依賴公式的白話寫在這裡。 -->
