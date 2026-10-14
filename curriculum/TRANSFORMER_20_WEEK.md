# Transformer 深度課程：20 週學習計畫

> 對齊 DeepLearning.AI 的 Deep Learning Specialization（DLS）相關單元，並參考 *Transformers in Practice*；Chronos 時序主線、校正、Rust、Candle 與決策服務是本計畫自行延伸的實作。

**適用對象**：具備軟體／系統工程經驗，想補齊深度學習與 Transformer 原理，並用 Rust 建立推論服務的人。

**時程**：W0–W19，共 20 週；其中 18 週新內容、W9 與 W19 為緩衝週。每週預估 6–8 小時，可以切成 25 分鐘的學習時段。

**每週最低交付**：完成表中的「必做產出」，並用幾句自己的話回答該週的關鍵問題。Python 用來快速驗證數學概念；Rust/Candle 用來理解實作與推論。表中的延伸項目視時間選做，不要求每週用兩種語言重寫同一套系統。若當週只有 3 小時，先做必做產出並記錄卡住的地方。

## 主線：從時間序列到型別化決策

最終要做的是：**輸入 metrics 時間序列 → 單次 forward → 輸出型別化決策**（例如 `Severity`、`Action`、`Confidence`）。

- **為什麼可以快**：逐 token 生成 N 個 token 需要依序跑約 N 個 decode step；分類式判斷只需一次 prefill／forward，再由 classification head 或候選 logits 給出結果。這個差距要在 W13 親手量測，不預設倍數。
- **Chronos 的角色與尺度保留（Scale Retention）**：初代 Chronos（encoder–decoder）把連續數值經 mean scaling 縮放並分箱成固定詞表的 token；Chronos-2 改為 encoder-only，不再分箱，而是把標準化並經 arcsinh 轉換的連續值切成 patch 當作 token，一次 forward 輸出多步的 quantile 分布。它和決策服務做的是同一件事——放掉自迴歸生成，一次 forward 直接輸出分布，差別只在輸出型別：一個是連續值的分位數，一個是離散選項的機率。需特別注意：**兩代的正規化都會抹除絕對數值高低**（例如 CPU 穩定在 5% 與 95%，經 mean scaling 後的 bin token 幾乎相同；標準化後的值也幾乎相同）。預測任務會把尺度乘回去所以不受影響，但分類決策會受影響，因此決策服務必須將視窗的絕對統計特徵（如 `mean`、`max`、`std`）保留並帶入分類頭，不能只靠相對波形做判斷。
- **信心要能被驗證**：決策服務輸出的機率必須在無時序洩漏的資料集上校正過（W15），低信心時要能拒答，而不是只看準確率。

主線在各週的落點：W1 softmax 與 temperature → W4 時間軸切分 → W6 數值 tokenization 與尺度保留 → W9 標籤資料集凍結 → W10 組裝 Block → W11 encoder-only 與 Chronos-2 對照 → W12 定案輕量 Encoder → W13 一次 forward 對比逐 token 生成 → W15 特徵融合、訓練與校正 → W16 型別化決策服務。

## 學習路線

| 階段 | 週次 | 目標 |
| --- | --- | --- |
| 基礎 | W0–W4 | Rust、張量、softmax、梯度、訓練迴圈、正則化與防洩漏資料切分 |
| 序列模型與注意力 | W5–W8 | 從 RNN 走到數值 tokenization、尺度保留、attention 與多頭注意力 |
| 緩衝 | W9 | 補完前半段、整理概念、凍結時序標籤資料集與可執行範例 |
| Transformer 與應用 | W10–W15 | 組裝目標 Block、比較架構、理解生成、模型調整與機率校正 |
| 系統工程 | W16–W18 | 建立非阻塞型別化決策服務、量測延遲、打包部署 |
| 緩衝 | W19 | 完成驗收與回顧 |

## 每週安排

### 基礎：W0–W4

| 週次 | 主題與關鍵問題 | 必做產出 | 時間充裕時再做 |
| --- | --- | --- | --- |
| **W0** | Rust 基礎：所有權、借用、`Result`、trait。為什麼借用規則能阻止資料競爭？ | 一個可測試的小型 Rust 程式，含錯誤處理與單元測試。 | 泛型與模組拆分。 |
| **W1** | 矩陣、shape、啟動函數、softmax 與張量。矩陣乘法的輸入輸出 shape 如何推導？調整 temperature 時，softmax 的分布在數學上發生什麼變化？ | NumPy 驗證矩陣乘法與 broadcasting；實作減去最大值的穩定 softmax，畫出不同 temperature 的分布；Candle 建立張量並檢查 shape。 | 用 Rust slice 寫矩陣乘法並觀察 stride；用 SVD 做 rank-10 近似，為 W14 的 LoRA 建立低秩直覺。 |
| **W2** | 前向傳播、鏈式法則與反向傳播。單一權重的梯度如何從 loss 傳回？ | 在 Python 寫最小 scalar autograd，與手算或數值梯度比較。 | 將少數運算移植到 Rust；無須一週內完成通用 autograd 引擎。 |
| **W3** | Cross entropy、batch、epoch、step、SGD/Adam。`backward`、清除梯度與參數更新各做什麼？ | 用 PyTorch 訓練小型分類器（以 logits 搭配 cross entropy），記錄 loss 曲線與驗證集結果。 | 用 Candle 寫同一個小型訓練迴圈。 |
| **W4** | 過擬合、正則化、學習率與資料切分。訓練與推論時 dropout 為何不同？時序滑動視窗若用隨機切分（Random Split）會造成什麼資料洩漏？ | 在 W3 分類器上比較有／無正則化的訓練及驗證表現；實作依時間軸先後切分（Chronological Split）並加上 Purge Gap 的切分函式；gap 長度至少為「視窗長度 + 標籤前瞻長度」（標籤若依視窗之後 N 步決定，也要算進去）。 | 實驗 BatchNorm 或用 Candle 重現其中一個設定。 |

### 序列模型與注意力：W5–W8

| 週次 | 主題與關鍵問題 | 必做產出 | 時間充裕時再做 |
| --- | --- | --- | --- |
| **W5** | RNN、LSTM/GRU 與長距依賴。循環狀態如何傳遞，梯度為何可能消失？ | 用短序列實驗比較簡單 RNN 與一種門控模型；畫出或列出時間步的狀態流。 | 在 Rust 寫 RNN Cell。 |
| **W6** | 文字與數值的 tokenization、embedding 與尺度遺失問題。把連續值經 mean scaling 量化成離散 token（初代 Chronos 的做法），損失了什麼（特別是絕對高低載資訊）、換到了什麼？Chronos-2 改用連續值 patch，又換掉了什麼？ | 用現成 tokenizer 檢查同一句話的 token id；寫 Chronos 式 mean scaling + binning 的量化與反量化，量測一段 metrics 序列的還原誤差，並同步抽出視窗絕對尺度特徵（如 `log(mean + ε)`、`max`、`std`）；畫出 embedding lookup 的 shape。 | 手寫簡化 BPE，或用 Rust `tokenizers` 載入同一份詞表；比較「單一指標 binning + scale」與「多指標 Linear Projection」兩種輸入表徵。 |
| **W7** | Self-attention、encoder–decoder attention、scaled dot-product 與 mask。`QKᵀ` 的每個分數代表哪兩個位置的關係？Q 與 K/V 來自不同序列時有何不同？ | 用小矩陣實作 `softmax(QKᵀ / √dₖ + mask)V`，驗證 shape 與 causal mask 不會看見未來位置；以同一函式示範 Q 來自 decoder、K/V 來自 encoder 的 cross-attention shape。 | 用 Candle 重現計算並比較數值。 |
| **W8** | Multi-head attention 與位置資訊。多個頭如何切分、合併？若沒有位置資訊會怎樣？ | 實作或組裝多頭注意力子模組，檢查各頭與合併後的 shape；加入簡單位置編碼。 | 比較有／無位置編碼的輸出。此週不要求完整 Transformer Block。 |

### W9：緩衝與檢查點

- 補齊 W0–W8 尚未完成的必做產出；不要加入新主題。
- 用自己的話說明：RNN 的限制、encoder–decoder attention 與 self-attention 的差別、causal mask 的用途、數值 tokenization 的取捨（以及為何分類決策必須保留絕對尺度特徵）。
- 建立並凍結資料集：寫一個 metrics 標籤產生器（含正常基線、突波、緩慢漂移與對應 `Severity`/`Action`，可用合成或歷史資料），搭配 W4 的時間軸切分（Purge Gap ≥ 視窗長度 + 標籤前瞻長度），產出並固定 Train / Calibration / Golden Test 三份資料集，減輕 W15 單週負擔。
- 標註資料來源：若標籤由規則產生，模型學到的就是那套規則，合成資料上的準確率與 ECE 不能代表真實表現。Golden Test 至少要有一小份真實或人工標註的視窗，並與合成部分分開報告。
- 讓 W7–W8 的小範例能從乾淨環境執行，為後續組裝 Block 做準備。

### Transformer 與應用：W10–W15

| 週次 | 主題與關鍵問題 | 必做產出 | 時間充裕時再做 |
| --- | --- | --- | --- |
| **W10** | LayerNorm、FFN、殘差與完整 Transformer Block。殘差路徑對資訊和梯度有何幫助？ | 用已完成的注意力子模組組裝一個可前向運算的輕量 Encoder Block（先明確定義層命名，方便 W16 與 Rust 端對齊；結構於 W12 定案），測試輸入輸出 shape 與 mask。 | 比較 LayerNorm 與 RMSNorm；用 `candle-nn` 先行對齊實作同一個完整 Block。 |
| **W11** | Encoder-only、decoder-only 與自迴歸生成。分類和逐 token 生成需要什麼不同的資訊流？Chronos-2 為什麼可以是 encoder-only，它捨棄了什麼能力？ | 為同一個短序列展示雙向與 causal mask；做一次 next-token forward，說明分類 head 的輸出；用 Chronos-2 對一段 metrics 序列做預測，觀察 quantile 輸出的 shape。 | 實作簡單逐 token 生成迴圈，比較 greedy、temperature 與 top-k sampling。 |
| **W12** | RoPE 與現代位置編碼。旋轉後的 Q/K 如何讓注意力分數反映相對位置？ | 用小維度向量驗證 RoPE 的位置運算，並替換 W10 Block 的位置處理，定案自建輕量 Encoder 的結構。 | 在 Candle 實作可重用的 RoPE 函式，確保與 Python 版數值一致。 |
| **W13** | Prefill、decode 與 KV cache。何時能重用 K/V？如果任務只需要一次分類或決策，還需要自迴歸生成嗎？ | 對同一個小型 decoder 比較「每步重算前綴」與「使用 cache」的生成結果和延遲；再量測一次 forward 與生成 100 個 token 的時間差距。隨機權重即可驗證 cache 正確性，量測生成品質才需要載入預訓練小模型。 | 用 Candle 實作 cache；補量測記憶體用量。 |
| **W14** | 量化與 LoRA：兩種不同的模型調整方式。量化誤差與 LoRA 權重合併各有什麼代價？ | 分別做一個小型量化前後的誤差實驗，以及 `W + (α/r)BA` 合併前後的輸出比較。 | 用 Candle 實作 `LoRALinear`；比較模型大小與延遲。 |
| **W15** | 訓練、尺度特徵融合與機率校正。模型輸出 80% 信心時，實際準確率真的是 80% 嗎？ | 載入 W9 切好的無洩漏時序資料集，從頭訓練 W12 定案的輕量決策分類模型（將 encoder 池化輸出與視窗絕對 scale 特徵拼接後送入 classification head）；在 calibration set 上擬合 temperature scaling 並設定低信心拒答門檻，在 golden test set 上畫 reliability diagram、計算 ECE（合成與真實部分分開報告）。 | 用 LoRA adapter 微調一個預訓練小模型（例如 Chronos-2 的 encoder），比較合併前後輸出；以 Chronos-2 輸出的 quantile 加上絕對 scale 作為決策特徵，比較只給點預測的差異。 |

### 系統工程：W16–W18

| 週次 | 主題與關鍵問題 | 必做產出 | 時間充裕時再做 |
| --- | --- | --- | --- |
| **W16** | 權重匯出、Rust 推論與型別化決策服務。如何驗證 Python 與 Rust 使用的是同一組模型、前處理與權重？規則層與模型層各負責什麼？同步推論如何避免阻塞非同步事件迴圈？ | 匯出 W15 模型為 SafeTensors，在 Rust 以 `candle-nn` 對齊實作同一輕量 Encoder 並載入權重，以固定輸入驗證數值誤差在容差內；用 Axum、Serde 和 Candle 提供決策 API，輸出強型別的 `Severity`、`Action`、`Confidence`，套用 W15 的 temperature 與拒答門檻；透過 `Arc` 共享模型並以 `tokio::task::spawn_blocking`（或專用推論執行緒佇列）隔離同步計算，確保不阻塞 Tokio worker 與健康檢查；規則層（門檻、冷卻、熔斷）與模型分開；定義輸入驗證與錯誤回應。 | 壓測併發請求下的推論佇列延遲、吞吐與背壓（Backpressure）保護。 |
| **W17** | 延遲量測與瓶頸分析。前處理、模型推論與 HTTP 各花多少時間？ | 量測固定資料集、批次大小與硬體下的 p50/p95 延遲、吞吐與記憶體；分開報告冷啟動和暖機後結果。 | 在同一台 Mac 上比較 Candle/Metal、MLX 與 PyTorch/MPS；確認模型、精度和輸入一致。與呼叫雲端 LLM API 的延遲對照。 |
| **W18** | 打包與部署。如何讓模型、前處理參數和服務版本可重現？ | 建立可執行的部署包、CI 建置與一次端到端驗收；記錄二進位、模型、設定檔和容器映像各自大小。 | 探索 Linux CPU 的最小容器映像與二進位縮減。 |

### W19：緩衝與最終驗收

- 補齊 W16–W18 的必做產出，重跑固定輸入的正確性、校正和效能量測。
- 交付服務啟動說明、範例請求、模型來源與版本、校正報告（含時間軸切分與 Purge Gap 說明）、效能報告，以及尚未解決的限制。
- 回顧主線：用 W13 的實測數字說明，單次分類或決策為何不需要逐 token 生成；KV cache 的收益應以逐 token 生成任務驗證。
- 把各週的關鍵問題串成一篇從矩陣乘法到型別化決策服務的說明文。

## 與官方課程的關係

| 參考課程 | 官方內容 | 本計畫的對應 |
| --- | --- | --- |
| DLS Course 1：Neural Networks and Deep Learning | 神經網路、向量化與反向傳播 | W1–W2；Rust autograd 為自行延伸。 |
| DLS Course 2：Improving Deep Neural Networks | 優化、超參數、正則化與 softmax | W1 的 softmax、W3–W4；時序切分與 Candle 實作為自行延伸。 |
| DLS Course 5 Week 1 | RNN、LSTM/GRU 與序列生成 | W5。 |
| DLS Course 5 Week 2 | 詞向量與 embedding | W6 的 embedding 部分；數值 tokenizer 與合成資料為延伸。 |
| DLS Course 5 Week 3 | 序列到序列模型及其 attention | W7 的 encoder–decoder attention 部分。 |
| DLS Course 5 Week 4 | Self-attention、multi-head attention、Transformer | W7–W8、W10–W11。 |
| *Transformers in Practice* Module 1 | 自迴歸行為、sampling 與輸出控制 | W11、W13 的生成部分。 |
| *Transformers in Practice* Module 2 | Attention、位置編碼與模型層 | W7–W8、W10–W12。 |
| *Transformers in Practice* Module 3 | 量化、KV cache、FlashAttention 與推論部署 | W13–W14、W17–W18；FlashAttention 作為延伸閱讀。 |

*Transformers in Practice* 由 Sharon Zhou 授課，與 Andrew Ng 有對談；它是三個模組的課程。Chronos、校正（calibration）、LoRA、PEFT、SafeTensors、Candle、Axum 和 Rust 部署是本計畫的延伸主題，不應標成該課程的指定週次。

官方課綱：[Deep Learning Specialization](https://www.deeplearning.ai/specializations/deep-learning/)、[Course 5: Sequence Models](https://www.coursera.org/learn/nlp-sequence-models)、[Transformers in Practice](https://www.deeplearning.ai/courses/transformers-in-practice)。課程單元可能更新，實際觀看時以官方頁面為準。

## 驗收與範圍

**核心交付**：可重現的小型時序決策服務（metrics 時序 + 尺度特徵 → 單次 forward → 型別化決策），具固定輸入的正確性檢查、記錄無洩漏 calibration set 與 ECE 的校正報告（註明標籤來源，合成與真實資料分開報告），以及記錄量測條件的效能報告。要比較模型方案，請固定資料集、前處理參數、輸入長度、批次大小、精度和硬體；跨框架結果需說明無法完全一致的部分。

**效能目標**：先量測，再設定服務延遲與大小預算。原先的「毫秒級」與「二進位小於 30 MB」可當探索目標；應明確區分 p50/p95 延遲、冷啟動時間、二進位、模型檔與容器映像大小，避免把未驗證的數字當成保證。

**部署平台**：Mac 上的 Metal/MLX/MPS 對比與 Linux 容器部署是兩個不同環境。若使用 Linux `scratch` 容器，需另行驗證靜態連結、執行時依賴、憑證與模型資產；Mac 的 Metal 執行檔不能直接作為 Linux 容器產物。

**Chronos 與自建 Candle 模型的職責分工**：Candle 載入 SafeTensors 前，Rust 端必須已有對應的模型架構實作。本計畫將 Chronos-2 定位為 Python 端的 zero-shot 與 quantile 對照基準；Rust 端則部署 W10–W15 自行定義並以 `candle-nn` 對齊實作的輕量 Encoder 分類器（2–4 層），確保雙邊權重命名、結構與尺度特徵融合完全可控，不依賴 `candle-transformers` 現成的大型預訓練模型支援度。

**三種「量化」與尺度保留要分清楚**：BPE 是把文字切成 token 並映射到 ID；初代 Chronos 的 binning 是把連續數值經 mean scaling 縮放後分箱成 token，屬於數值 tokenization，在 W6 處理（Chronos-2 不再分箱，改用連續值 patch；兩者都必須額外保留視窗絕對 scale 特徵，避免抹除高低載差異）；模型權重量化是降低參數精度，在 W14 處理。W6 應觀察分詞長度、詞表覆蓋、解碼結果、數值還原誤差以及被抹除的絕對尺度資訊。

**資料夾安排**：每週練習放在 `weeks/wNN-主題/`，開始該週時再建立，不預先建立空目錄。會被後續週次重用的程式（W4 切分、W6 量化與尺度特徵、W9 標籤產生器、W10–W15 Encoder）整理進共用套件 `py/tsdecide/`；W16–W18 的決策服務放在 `crates/decision-service/`；W9 凍結的資料集與 W15 的權重放在 `data/`、`models/`，不進 git。
