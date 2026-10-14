# Transformer 20 週學習計畫

從矩陣乘法一路走到「metrics 時序 → 單次 forward → 型別化決策」的 Rust 推論服務。課綱見 [`curriculum/TRANSFORMER_20_WEEK.md`](curriculum/TRANSFORMER_20_WEEK.md)。

## 資料夾

| 路徑 | 內容 |
| --- | --- |
| [`curriculum/`](curriculum/) | 20 週課綱 |
| [`weeks/`](weeks/) | 每週練習（`wNN-主題/`：README、notebook、選做的 `rust/`） |
| [`py/tsdecide/`](py/tsdecide/) | 跨週共用的 Python 套件：W4 切分、W6 量化與尺度特徵、W9 標籤產生器、W10–W15 Encoder |
| [`crates/`](crates/) | Rust crates：`rust_mastery`（語法章節練習），W16 起的 `decision-service` |
| `data/`、`models/` | W9 凍結的資料集與 W15 訓練出的權重（不進 git） |

## 環境

```bash
# Python：啟用虛擬環境，並以可編輯模式安裝共用套件
source pyenv/bin/activate
pip install -e .

# Rust：根目錄是 Cargo workspace，自動包含 crates/*；各週的 rust/ 需手動加進 members
source "$HOME/.cargo/env"
cargo run -p rust_mastery
```
