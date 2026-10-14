//! 練習 2：借用——只讀用 `&[f64]`，要改用 `&mut`。

/// 平均值；空視窗回傳 `None`（不要回傳 NaN）。
pub fn mean(window: &[f64]) -> Option<f64> {
    // TODO
    todo!()
}

/// 最大值；空視窗回傳 `None`。提示：`iter().copied().reduce(f64::max)`。
pub fn max(window: &[f64]) -> Option<f64> {
    // TODO
    todo!()
}

/// 把 `value` 推進固定長度的滑動視窗：超過 `cap` 時丟掉最舊的。
/// 參數是 `&mut Vec<f64>`：呼叫端保有所有權，這裡只是暫時借來改。
pub fn push_window(window: &mut Vec<f64>, value: f64, cap: usize) {
    // TODO
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_and_max() {
        let w = [1.0, 2.0, 6.0];
        assert_eq!(mean(&w), Some(3.0));
        assert_eq!(max(&w), Some(6.0));
    }

    #[test]
    fn empty_window_is_none() {
        assert_eq!(mean(&[]), None);
        assert_eq!(max(&[]), None);
    }

    #[test]
    fn window_drops_oldest() {
        let mut w = Vec::new();
        for v in [1.0, 2.0, 3.0, 4.0] {
            push_window(&mut w, v, 3);
        }
        assert_eq!(w, vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn caller_keeps_ownership() {
        let mut w = vec![5.0];
        push_window(&mut w, 7.0, 10);
        // 借用結束後，w 仍然屬於這裡，可以繼續使用
        assert_eq!(mean(&w), Some(6.0));
    }
}
