use pinyin::ToPinyin;

/// 获取拼音的全拼和缩写
pub fn get_pinyin_variants(name: &str) -> (String, String) {
    let mut full = String::new();
    let mut abbr = String::new();

    for ch in name.chars() {
        if let Some(p) = ch.to_pinyin() {
            let p_str = p.plain().to_string();
            full.push_str(&p_str);
            abbr.push(p_str.chars().next().unwrap_or_default());
        } else {
            // 非中文字符（保留原字符）
            full.push(ch);
            abbr.push(ch);
        }
    }

    (full.to_lowercase(), abbr.to_lowercase())
}
