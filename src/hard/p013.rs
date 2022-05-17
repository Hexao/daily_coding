use std::collections::HashSet;

/// Given an integer `k` and a string `s`, find the longest substring that
/// contains at most `k` distinct characters.
///
/// For exemple, given `s = "abcba"` and `k = 2`, the longest substring
/// with `k` distince characters is `"bcb"`.
pub fn solve<'str>(data: &'str str, distinct: usize) -> &'str str {
    assert!(distinct > 0);

    let mut h_set = HashSet::new();
    let mut stats = (0, 1);
    let data_len = data.len();
    let mut start = 0;

    while start < data_len {
        let mut i = 0;

        while let Some(c) = data.chars().nth(start + i) {
            if h_set.insert(c) && h_set.len() > distinct {
                break;
            } else {
                i += 1;
            }
        }

        if stats.1 < i {
            stats = (start, i);
        }

        start += 1;
        h_set.clear();
    }

    let (start, len) = stats;
    data.get(start..start + len).unwrap()
}
