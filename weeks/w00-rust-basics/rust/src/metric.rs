//! 練習 1：所有權與 `Result`——把一行文字解析成 [`Sample`]。

use std::fmt;

/// 一筆 metrics 樣本。`name` 是 `String`（擁有資料），所以 `Sample` 可以活得比輸入的 `&str` 久。
#[derive(Debug, Clone, PartialEq)]
pub struct Sample {
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// 找不到 `=`，例如 `"cpu 93.5"`。
    MissingEquals,
    /// `=` 左邊是空的（去掉空白後），例如 `"=93.5"`。
    EmptyName,
    /// 右邊不是合法的有限數字，例如 `"cpu=abc"`、`"cpu=NaN"`；保留原始字串方便除錯。
    BadValue(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: 為每個變體寫一句人看得懂的訊息，BadValue 要帶出原始字串
        todo!()
    }
}

impl std::error::Error for ParseError {}

/// 解析 `"name=value"`；兩邊的空白要去掉，值必須是有限數字（不接受 NaN、inf）。
///
/// 提示：`str::split_once`、`str::trim`、`str::parse::<f64>`、`f64::is_finite`、`map_err`、`?`。
pub fn parse_line(line: &str) -> Result<Sample, ParseError> {
    // TODO
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_name_and_value() {
        let s = parse_line("cpu=93.5").unwrap();
        assert_eq!(s, Sample { name: "cpu".into(), value: 93.5 });
    }

    #[test]
    fn trims_whitespace() {
        let s = parse_line("  mem = 42 ").unwrap();
        assert_eq!(s, Sample { name: "mem".into(), value: 42.0 });
    }

    #[test]
    fn sample_outlives_input() {
        let s = {
            let line = String::from("cpu=1");
            parse_line(&line).unwrap()
        }; // line 在這裡被釋放，s 仍然可用，因為 name 是自己擁有的 String
        assert_eq!(s.name, "cpu");
    }

    #[test]
    fn rejects_bad_input() {
        assert_eq!(parse_line("cpu 93.5"), Err(ParseError::MissingEquals));
        assert_eq!(parse_line(" =1"), Err(ParseError::EmptyName));
        assert_eq!(parse_line("cpu=abc"), Err(ParseError::BadValue("abc".into())));
        assert_eq!(parse_line("cpu=NaN"), Err(ParseError::BadValue("NaN".into())));
    }

    #[test]
    fn error_message_mentions_bad_value() {
        let msg = ParseError::BadValue("abc".into()).to_string();
        assert!(msg.contains("abc"), "訊息應包含原始字串：{msg}");
    }
}
