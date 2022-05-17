/// Given the mapping `[a = 1, b = 2, ..., z = 26]` and an encoded message,
/// count the number of ways it can be decoded.
///
/// For example, the message `'111'` would give `3`, since it could be
/// decoded as `'aaa'`, `'ka'`, and `'ak'`.
pub fn solve(data: &str) -> usize {
    let bytes = data.as_bytes();

    if bytes.iter().any(|b| !(b'0'..=b'9').contains(b)) {
        return 0;
    }

    let mut queue = Vec::with_capacity(10);
    let len = bytes.len();
    let mut valid = 0;
    queue.push(0);

    while !queue.is_empty() {
        let from = queue.pop().unwrap();

        let mut decoded = bytes[from] - b'0';
        if decoded == 0 { continue; }

        if from + 1 < len {
            queue.push(from + 1);
            if decoded > 2 { continue; }

            decoded = decoded * 10 + bytes[from + 1] - b'0';
            if decoded <= b'z' - b'a' + 1 {
                if from + 2 < len {
                    queue.push(from + 2);
                } else {
                    valid += 1;
                }
            }
        } else {
            valid += 1;
        }
    }

    valid
}
