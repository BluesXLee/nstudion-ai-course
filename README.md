# Transformer 20 週學習計畫

從矩陣乘法一路走到「metrics 時序 → 單次 forward → 型別化決策」的 Rust 推論服務。課綱見 [`curriculum/TRANSFORMER_20_WEEK.md`](curriculum/TRANSFORMER_20_WEEK.md)。

## 資料夾

| 路徑 | 內容 |
| --- | --- |
| [`curriculum/`](curriculum/) | 20 週課綱 |
| [`week-00/`](week-00/) … | 每週的 README、notebook 與 Rust 練習 |
| [`docs/`](docs/) | Rust 參考指南（給 Python／Go 開發者） |
| [`rust_mastery/`](rust_mastery/) | Rust 語法章節練習（ch06–ch11） |
| [`lab/`](lab/) | 課綱以外的實驗 |

## 環境

```bash
# Python
source pyenv/bin/activate

# Rust（根目錄是 Cargo workspace，-p 後面接套件名稱）
source "$HOME/.cargo/env"
cargo run -p rust_course    # W0：week-00/rust_course
cargo run -p rust_matrix    # W1：week-01/rust_course
cargo run -p rust_mastery
```
