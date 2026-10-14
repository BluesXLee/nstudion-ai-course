# 每週練習

每週一個資料夾，命名為 `wNN-主題`（例如 `w01-matrix-softmax`），開始該週時再建立，不預先建立空目錄。

```
wNN-主題/
  README.md      該週關鍵問題的回答與卡住的地方
  *.ipynb        Python 驗證
  rust/          選做；建立後把路徑加進根目錄 Cargo.toml 的 members
```

會被後續週次重複使用的程式（切分、量化、標籤產生器、模型）不要留在這裡，請整理進 [`py/tsdecide/`](../py/tsdecide/)，notebook 用 `from tsdecide import ...` 引用。
