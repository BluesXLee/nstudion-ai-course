//! 從 stdin 逐行讀 `name=value`，維護最近 5 筆的視窗並印出嚴重度。
//!
//! ```bash
//! printf 'cpu=50\ncpu=95\ncpu=oops\ncpu=99.5\n' | cargo run -p w00_rust_basics
//! ```

use std::io::{self, BufRead};

use w00_rust_basics::metric::parse_line;
use w00_rust_basics::severity::{MeanThreshold, SpikeRule, worst};
use w00_rust_basics::stats::push_window;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mean_rule = MeanThreshold { warn: 70.0, critical: 90.0 };
    let spike_rule = SpikeRule { limit: 99.0 };
    let mut window = Vec::new();

    for (i, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        match parse_line(&line) {
            Ok(sample) => {
                push_window(&mut window, sample.value, 5);
                let sev = worst(&[&mean_rule, &spike_rule], &window);
                println!("{} = {:>6.1}  window={:?}  -> {:?}", sample.name, sample.value, window, sev);
            }
            // 單行壞資料不讓整個程式停掉：記錄後跳過
            Err(e) => eprintln!("line {}: {e}", i + 1),
        }
    }
    Ok(())
}
