//! # vs_rot13
//! `vs_rot13` 是一个ROT13转码工具

/// 将给定的字符串进行ROT13转码
///
/// # Examples
///
/// ```
/// use vs_rot13::rot13;
///
/// assert_eq!("uryyb,ebg13!", rot13("hello,rot13!"));
/// assert_eq!("Test", rot13(&rot13("Test")));
/// ```
pub fn rot13(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            'A'..='M' | 'a'..='m' => ((c as u8) + 13) as char,
            'N'..='Z' | 'n'..='z' => ((c as u8) - 13) as char,
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot13() {
        assert_eq!("uryyb,ebg13!", rot13("hello,rot13!"));
        assert_eq!("Test", rot13(&rot13("Test")));
    }
}
