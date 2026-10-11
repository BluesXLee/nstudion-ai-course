# Transformer 深度課程：20 週學習計畫

> 對齊 DeepLearning.AI 的 Deep Learning Specialization（DLS）相關單元，並參考 *Transformers in Practice*；Chronos 時序主線、校正、Rust、Candle 與決策服務是本計畫自行延伸的實作。

**適用對象**：具備軟體／系統工程經驗，想補齊深度學習與 Transformer 原理，並用 Rust 建立推論服務的人。

**時程**：W0–W19，共 20 週；其中 18 週新內容、W9 與 W19 為緩衝週。每週預估 6–8 小時，可以切成 25 分鐘的學習時段。

**每週最低交付**：完成表中的「必做產出」，並用幾句自己的話回答該週的關鍵問題。Python 用來快速驗證數學概念；Rust/Candle 用來理解實作與推論。表中的延伸項目視時間選做，不要求每週用兩種語言重寫同一套系統。若當週只有 3 小時，先做必做產出並記錄卡住的地方。

## 主線：從時間序列到型別化決策

最終要做的是：**輸入 metrics 時間序列 → 單次 forward → 輸出型別化決策**（例如 `Severity`、`Action`、`Confidence`）。

- **為什麼可以快**：逐 token 生成 N 個 token 需要依序跑約 N 個 decode step；分類式判斷只需一次 prefill／forward，再由 classification head 或候選 logits 給出結果。這個差距要在 W13 親手量測，不預設倍數。
- **Chronos 的角色**：Chronos 把連續數值縮放並分箱成 token，交給 Transformer 處理；Chronos-2 是 encoder-only，一次 forward 輸出多步的 quantile 分布。它和決策服務做的是同一件事——放掉自迴歸生成，一次 forward 直接輸出分布，差別只在輸出型別：一個是連續值的分位數，一個是離散選項的機率。
- **信心要能被驗證**：決策服務輸出的機率必須校正過（W15），低信心時要能拒答，而不是只看準確率。

主線在各週的落點：W1 softmax 與 temperature → W6 數值 tokenization → W11 encoder-only 與 Chronos-2 → W13 一次 forward 對比逐 token 生成 → W15 微調與校正 → W16 型別化決策服務。

## 學習路線

| 階段 | 週次 | 目標 |
| --- | --- | --- |
| 基礎 | W0–W4 | Rust、張量、softmax、梯度、訓練迴圈與正則化 |
| 序列模型與注意力 | W5–W8 | 從 RNN 走到 tokenization、attention 與多頭注意力 |
| 緩衝 | W9 | 補完前半段、整理概念與可執行範例 |
| Transformer 與應用 | W10–W15 | 組裝 Block、比較架構、理解生成、模型調整與校正 |
| 系統工程 | W16–W18 | 建立型別化決策服務、量測延遲、部署 |
| 緩衝 | W19 | 完成驗收與回顧 |

## 每週安排

### 基礎：W0–W4

| 週次 | 主題與關鍵問題 | 必做產出 | 時間充裕時再做 |
| --- | --- | --- | --- |
| **W0** | Rust 基礎：所有權、借用、`Result`、trait。為什麼借用規則能阻止資料競爭？ | 一個可測試的小型 Rust 程式，含錯誤處理與單元測試。 | 泛型與模組拆分。 |
| **W1** | 矩陣、shape、啟動函數、softmax 與張量。矩陣乘法的輸入輸出 shape 如何推導？調整 temperature 時，softmax 的分布在數學上發生什麼變化？ | NumPy 驗證矩陣乘法與 broadcasting；實作減去最大值的穩定 softmax，畫出不同 temperature 的分布；Candle 建立張量並檢查 shape。 | 用 Rust slice 寫矩陣乘法並觀察 stride；用 SVD 做 rank-10 近似，為 W14 的 LoRA 建立低秩直覺。 |
| **W2** | 前向傳播、鏈式法則與反向傳播。單一權重的梯度如何從 loss 傳回？ | 在 Python 寫最小 scalar autograd，與手算或數值梯度比較。 | 將少數運算移植到 Rust；無須一週內完成通用 autograd 引擎。 |
| **W3** | Cross entropy、batch、epoch、step、SGD/Adam。`backward`、清除梯度與參數更新各做什麼？ | 用 PyTorch 訓練小型分類器（以 logits 搭配 cross entropy），記錄 loss 曲線與驗證集結果。 | 用 Candle 寫同一個小型訓練迴圈。 |
| **W4** | 過擬合、正則化、學習率與資料切分。訓練與推論時 dropout 為何不同？ | 在 W3 分類器上比較有／無正則化的訓練及驗證表現。 | 實驗 BatchNorm 或用 Candle 重現其中一個設定。 |

### 序列模型與注意力：W5–W8

| 週次 | 主題與關鍵問題 | 必做產出 | 時間充裕時再做 |
| --- | --- | --- | --- |
| **W5** | RNN、LSTM/GRU 與長距依賴。循環狀態如何傳遞，梯度為何可能消失？ | 用短序列實驗比較簡單 RNN 與一種門控模型；畫出或列出時間步的狀態流。 | 在 Rust 寫 RNN Cell。 |
| **W6** | 文字與數值的 tokenization、embedding。把連續值量化成離散 token，損失了什麼、換到了什麼？ | 用現成 tokenizer 檢查同一句話的 token id；寫 Chronos 式 mean scaling + binning 的量化與反量化，量測一段 metrics 序列的還原誤差；畫出 embedding lookup 的 shape。 | 手寫簡化 BPE，或用 Rust `tokenizers` 載入同一份詞表。 |
| **W7** | Self-attention、encoder–decoder attention、scaled dot-product 與 mask。`QKᵀ` 的每個分數代表哪兩個位置的關係？Q 與 K/V 來自不同序列時有何不同？ | 用小矩陣實作 `softmax(QKᵀ / √dₖ + mask)V`，驗證 shape 與 causal mask 不會看見未來位置；以同一函式示範 Q 來自 decoder、K/V 來自 encoder 的 cross-attention shape。 | 用 Candle 重現計算並比較數值。 |
| **W8** | Multi-head attention 與位置資訊。多個頭如何切分、合併？若沒有位置資訊會怎樣？ | 實作或組裝多頭注意力子模組，檢查各頭與合併後的 shape；加入簡單位置編碼。 | 比較有／無位置編碼的輸出。此週不要求完整 Transformer Block。 |

### W9：緩衝與檢查點

- 補齊 W0–W8 尚未完成的必做產出；不要加入新主題。
- 用自己的話說明：RNN 的限制、encoder–decoder attention 與 self-attention 的差別、causal mask 的用途、數值 tokenization 的取捨。
- 讓 W7–W8 的小範例能從乾淨環境執行，為後續組裝 Block 做準備。

### Transformer 與應用：W10–W15

| 週次 | 主題與關鍵問題 | 必做產出 | 時間充裕時再做 |
| --- | --- | --- | --- |
| **W10** | LayerNorm、FFN、殘差與完整 Transformer Block。殘差路徑對資訊和梯度有何幫助？ | 用已完成的注意力子模組組裝一個可前向運算的 Block，測試輸入輸出 shape 與 mask。 | 比較 LayerNorm 與 RMSNorm；用 Candle 重現完整 Block，或做一次小型訓練。 |
| **W11** | Encoder-only、decoder-only 與自迴歸生成。分類和逐 token 生成需要什麼不同的資訊流？Chronos-2 為什麼可以是 encoder-only，它捨棄了什麼能力？ | 為同一個短序列展示雙向與 causal mask；做一次 next-token forward，說明分類 head 的輸出；用 Chronos-2 對一段 metrics 序列做預測，觀察 quantile 輸出的 shape。 | 實作簡單逐 token 生成迴圈，比較 greedy、temperature 與 top-k sampling。 |
| **W12** | RoPE 與現代位置編碼。旋轉後的 Q/K 如何讓注意力分數反映相對位置？ | 用小維度向量驗證 RoPE 的位置運算，並替換 W10 Block 的位置處理。 | 在 Candle 實作可重用的 RoPE 函式。 |
| **W13** | Prefill、decode 與 KV cache。何時能重用 K/V？如果任務只需要一次分類或決策，還需要自迴歸生成嗎？ | 對同一個小型 decoder 比較「每步重算前綴」與「使用 cache」的生成結果和延遲；再量測一次 forward 與生成 100 個 token 的時間差距。隨機權重即可驗證 cache 正確性，量測生成品質才需要載入預訓練小模型。 | 用 Candle 實作 cache；補量測記憶體用量。 |
| **W14** | 量化與 LoRA：兩種不同的模型調整方式。量化誤差與 LoRA 權重合併各有什麼代價？ | 分別做一個小型量化前後的誤差實驗，以及 `W + (α/r)BA` 合併前後的輸出比較。 | 用 Candle 實作 `LoRALinear`；比較模型大小與延遲。 |
| **W15** | 微調與校正。模型輸出 80% 信心時，實際準確率真的是 80% 嗎？ | 用 metrics 時序資料微調或訓練一個小型決策分類模型；切出 calibration set 與 golden test set，畫 reliability diagram、計算 ECE，套用 temperature scaling 並設定低信心拒答門檻。 | 用 LoRA adapter 微調並比較合併前後輸出；以 Chronos-2 輸出的 quantile 作為決策特徵，比較只給點預測的差異。 |

### 系統工程：W16–W18

| 週次 | 主題與關鍵問題 | 必做產出 | 時間充裕時再做 |
| --- | --- | --- | --- |
| **W16** | 權重匯出、Rust 推論與型別化決策服務。如何驗證 Python 與 Rust 使用的是同一組模型與 tokenizer？規則層與模型層各負責什麼？ | 匯出 W15 模型為 SafeTensors，在 Rust/Candle 載入並以固定輸入比較輸出（設定數值容差）；用 Axum、Serde 和 Candle 提供決策 API，輸出強型別的 `Severity`、`Action`、`Confidence`，套用 W15 的 temperature 與拒答門檻；規則層（門檻、冷卻、熔斷）與模型分開；啟動時載入模型，定義輸入驗證、錯誤回應與健康檢查。 | 壓測併發請求，檢查模型共享與阻塞行為。 |
| **W17** | 延遲量測與瓶頸分析。前處理、模型推論與 HTTP 各花多少時間？ | 量測固定資料集、批次大小與硬體下的 p50/p95 延遲、吞吐與記憶體；分開報告冷啟動和暖機後結果。 | 在同一台 Mac 上比較 Candle/Metal、MLX 與 PyTorch/MPS；確認模型、精度和輸入一致。與呼叫雲端 LLM API 的延遲對照。 |
| **W18** | 打包與部署。如何讓模型、tokenizer 和服務版本可重現？ | 建立可執行的部署包、CI 建置與一次端到端驗收；記錄二進位、模型、tokenizer 和容器映像各自大小。 | 探索 Linux CPU 的最小容器映像與二進位縮減。 |

### W19：緩衝與最終驗收

- 補齊 W16–W18 的必做產出，重跑固定輸入的正確性、校正和效能量測。
- 交付服務啟動說明、範例請求、模型來源與版本、校正報告、效能報告，以及尚未解決的限制。
- 回顧主線：用 W13 的實測數字說明，單次分類或決策為何不需要逐 token 生成；KV cache 的收益應以逐 token 生成任務驗證。
- 把各週的關鍵問題串成一篇從矩陣乘法到型別化決策服務的說明文。

## 與官方課程的關係

| 參考課程 | 官方內容 | 本計畫的對應 |
| --- | --- | --- |
| DLS Course 1：Neural Networks and Deep Learning | 神經網路、向量化與反向傳播 | W1–W2；Rust autograd 為自行延伸。 |
| DLS Course 2：Improving Deep Neural Networks | 優化、超參數、正則化與 softmax | W1 的 softmax、W3–W4；Candle 實作為自行延伸。 |
| DLS Course 5 Week 1 | RNN、LSTM/GRU 與序列生成 | W5。 |
| DLS Course 5 Week 2 | 詞向量與 embedding | W6 的 embedding 部分；tokenizer 實作為延伸。 |
| DLS Course 5 Week 3 | 序列到序列模型及其 attention | W7 的 encoder–decoder attention 部分。 |
| DLS Course 5 Week 4 | Self-attention、multi-head attention、Transformer | W7–W8、W10–W11。 |
| *Transformers in Practice* Module 1 | 自迴歸行為、sampling 與輸出控制 | W11、W13 的生成部分。 |
| *Transformers in Practice* Module 2 | Attention、位置編碼與模型層 | W7–W8、W10–W12。 |
| *Transformers in Practice* Module 3 | 量化、KV cache、FlashAttention 與推論部署 | W13–W14、W17–W18；FlashAttention 作為延伸閱讀。 |

*Transformers in Practice* 由 Sharon Zhou 授課，與 Andrew Ng 有對談；它是三個模組的課程。Chronos、校正（calibration）、LoRA、PEFT、SafeTensors、Candle、Axum 和 Rust 部署是本計畫的延伸主題，不應標成該課程的指定週次。

官方課綱：[Deep Learning Specialization](https://www.deeplearning.ai/specializations/deep-learning/)、[Course 5: Sequence Models](https://www.coursera.org/learn/nlp-sequence-models)、[Transformers in Practice](https://www.deeplearning.ai/courses/transformers-in-practice)。課程單元可能更新，實際觀看時以官方頁面為準。

## 驗收與範圍

**核心交付**：可重現的小型時序決策服務（metrics 時序 → 單次 forward → 型別化決策），具固定輸入的正確性檢查、記錄 calibration set 與 ECE 的校正報告，以及記錄量測條件的效能報告。要比較模型方案，請固定資料集、tokenizer、輸入長度、批次大小、精度和硬體；跨框架結果需說明無法完全一致的部分。

**效能目標**：先量測，再設定服務延遲與大小預算。原先的「毫秒級」與「二進位小於 30 MB」可當探索目標；應明確區分 p50/p95 延遲、冷啟動時間、二進位、模型檔與容器映像大小，避免把未驗證的數字當成保證。

**部署平台**：Mac 上的 Metal/MLX/MPS 對比與 Linux 容器部署是兩個不同環境。若使用 Linux `scratch` 容器，需另行驗證靜態連結、執行時依賴、憑證與模型資產；Mac 的 Metal 執行檔不能直接作為 Linux 容器產物。

**Chronos 與 Candle 的相容性**：Candle 載入 SafeTensors 前，Rust 端必須已有對應的模型架構實作。W15 選模型前，先確認 `candle-transformers` 是否支援該架構；若 Chronos-2 沒有現成的 Candle 實作，就把它當成 Python 端的對照基準，Rust 端部署自行訓練、且 Candle 已支援架構的小型 encoder 分類器。

**三種「量化」要分清楚**：BPE 是把文字切成 token 並映射到 ID；Chronos 式 binning 是把連續數值縮放後分箱成 token，屬於 tokenization，W6 處理；模型權重量化是降低參數精度，W14 處理。W6 應觀察分詞長度、詞表覆蓋、解碼結果與數值還原誤差。

**資料夾安排**：目前倉庫已有 `week-01/` 等內容。後續實作可以沿用現有資料夾，按週新增 README、程式碼及必要測試；無須預先建立所有週的空目錄。
