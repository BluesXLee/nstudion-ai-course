//! 練習 3：trait——讓不同的判斷規則有同一個介面。W16 會再加一個「模型」實作。

use crate::stats;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Ok,
    Warn,
    Critical,
}

pub trait Rule {
    /// 根據視窗判斷嚴重度；空視窗一律回傳 `Severity::Ok`。
    fn evaluate(&self, window: &[f64]) -> Severity;
}

/// 用視窗平均值比門檻：`>= critical` 是 Critical，`>= warn` 是 Warn，其餘 Ok。
pub struct MeanThreshold {
    pub warn: f64,
    pub critical: f64,
}

impl Rule for MeanThreshold {
    fn evaluate(&self, window: &[f64]) -> Severity {
        // TODO: 用 stats::mean
        todo!()
    }
}

/// 只要視窗中任一點 `>= limit` 就是 Critical，否則 Ok——抓突波用。
pub struct SpikeRule {
    pub limit: f64,
}

impl Rule for SpikeRule {
    fn evaluate(&self, window: &[f64]) -> Severity {
        // TODO: 用 stats::max
        todo!()
    }
}

/// 套用所有規則，取最嚴重的結果。`&[&dyn Rule]` 讓不同型別的規則放進同一個 slice。
pub fn worst(rules: &[&dyn Rule], window: &[f64]) -> Severity {
    // TODO: 提示 Severity 有 derive Ord
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MEAN: MeanThreshold = MeanThreshold { warn: 70.0, critical: 90.0 };
    const SPIKE: SpikeRule = SpikeRule { limit: 99.0 };

    #[test]
    fn mean_threshold_levels() {
        assert_eq!(MEAN.evaluate(&[10.0, 20.0]), Severity::Ok);
        assert_eq!(MEAN.evaluate(&[70.0, 70.0]), Severity::Warn);
        assert_eq!(MEAN.evaluate(&[95.0, 85.0]), Severity::Critical);
    }

    #[test]
    fn empty_window_is_ok() {
        assert_eq!(MEAN.evaluate(&[]), Severity::Ok);
        assert_eq!(SPIKE.evaluate(&[]), Severity::Ok);
    }

    #[test]
    fn spike_catches_single_point() {
        // 平均只有 ~35，MeanThreshold 會漏掉，SpikeRule 抓得到
        let w = [5.0, 5.0, 99.5];
        assert_eq!(MEAN.evaluate(&w), Severity::Ok);
        assert_eq!(SPIKE.evaluate(&w), Severity::Critical);
        assert_eq!(worst(&[&MEAN, &SPIKE], &w), Severity::Critical);
    }

    #[test]
    fn worst_of_no_rules_is_ok() {
        assert_eq!(worst(&[], &[50.0]), Severity::Ok);
    }
}
