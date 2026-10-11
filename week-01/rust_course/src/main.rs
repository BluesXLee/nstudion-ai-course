use anyhow::Result;
use candle_core::{Device, Tensor};

// =========================================================================
// Week 1 實戰練習：矩陣、連續記憶體與秩 (Rank)
//
// 目標：親手寫出底層記憶體索引，理解 Tensor 的 Shape 與 Stride，
//       並透過單元測試驗證每個數字的來源。
// =========================================================================

/// 任務 1：手寫向量內積 (Dot Product)
///
/// 輸入：兩個切片 a: &[f32], b: &[f32]
/// 輸出：純量總和 f32
/// 規則：
/// 1. 若兩者長度不符，請用 anyhow::bail!("長度不匹配") 拋出錯誤
/// 2. 計算 sum(a[i] * b[i])
pub fn dot_product(a: &[f32], b: &[f32]) -> Result<f32> {
    // TODO: 請親手實作這裡
    todo!("請實作向量內積")
}

/// 任務 2：手寫二維矩陣乘法（連續記憶體 Flat Buffer）
///
/// 輸入：
/// - a: &[f32], shape_a: (m, k1)
/// - b: &[f32], shape_b: (k2, p)
/// 輸出：
/// - (Vec<f32>, (m, p))
///
/// 核心知識點：
/// 在真實系統與 C/Rust/CUDA 中，矩陣是一維連續儲存的（Row-major）。
/// 第 (i, j) 個元素在記憶體中的 index 公式為：i * cols + j
pub fn matmul_flat(
    a: &[f32],
    shape_a: (usize, usize),
    b: &[f32],
    shape_b: (usize, usize),
) -> Result<(Vec<f32>, (usize, usize))> {
    let (m, k1) = shape_a;
    let (k2, p) = shape_b;

    // TODO: 1. 檢查內側維度是否相同，不同則報錯
    // TODO: 2. 建立大小為 m * p 的 Vec<f32>，初始值為 0.0
    // TODO: 3. 用三層迴圈算出每個位置的值，並正確計算 flat index
    todo!("請實作矩陣乘法")
}

/// 任務 3：手寫外積 (Outer Product)
///
/// 輸入：u: &[f32] (長度 m), v: &[f32] (長度 p)
/// 輸出：(Vec<f32>, (m, p))
///
/// 觀察重點：
/// 外積產生的矩陣 rank 必為 1，因為每一列都只是向量 v 的純量倍數。
pub fn outer_product(u: &[f32], v: &[f32]) -> (Vec<f32>, (usize, usize)) {
    // TODO: 請實作外積計算
    todo!("請實作外積")
}

/// 任務 4：用 Hugging Face Candle 完成矩陣乘法
///
/// 提示：
/// - 建立 CPU 設備：Device::Cpu
/// - 建立 Tensor：Tensor::from_vec(vec_data, shape, &device)?
/// - 矩陣相乘：tensor_a.matmul(&tensor_b)?
pub fn candle_matmul_demo() -> Result<()> {
    let device = Device::Cpu;

    // A: (2, 3)
    let a_data = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0];
    let a_shape = (2, 3);

    // B: (3, 2)
    let b_data = vec![7.0f32, 8.0, 9.0, 10.0, 11.0, 12.0];
    let b_shape = (3, 2);

    // TODO: 請用 Candle API 建立 tensor_a 與 tensor_b
    // TODO: 呼叫 matmul 算出 tensor_c
    // TODO: 印出 tensor_c 的 shape 與 stride

    todo!("請實作 Candle Tensor 矩陣乘法")
}

fn main() -> Result<()> {
    println!("請執行 `cargo test -p rust_matrix` 來逐步完成測試！");
    Ok(())
}

// =========================================================================
// 單元測試：跑 `cargo test -p rust_matrix` 逐一攻克
// =========================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32.0
        let result = dot_product(&a, &b).unwrap();
        assert_eq!(result, 32.0);

        let c = [1.0, 2.0];
        assert!(dot_product(&a, &c).is_err(), "維度不合應該要報錯");
    }

    #[test]
    fn test_matmul_flat() {
        // A: 2x3
        let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        // B: 3x2
        let b = [7.0, 8.0, 9.0, 10.0, 11.0, 12.0];

        let (c, shape) = matmul_flat(&a, (2, 3), &b, (3, 2)).unwrap();
        assert_eq!(shape, (2, 2));
        assert_eq!(c, vec![58.0, 64.0, 139.0, 154.0]);

        // 維度錯誤測試
        assert!(matmul_flat(&a, (2, 3), &b, (2, 3)).is_err());
    }

    #[test]
    fn test_outer_product() {
        let u = [1.0, 2.0];
        let v = [10.0, 20.0, 30.0];
        let (res, shape) = outer_product(&u, &v);
        assert_eq!(shape, (2, 3));
        // 第一列：1 * [10, 20, 30]
        // 第二列：2 * [10, 20, 30]
        assert_eq!(res, vec![10.0, 20.0, 30.0, 20.0, 40.0, 60.0]);
    }
}
