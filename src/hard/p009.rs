/// Given a list of integers, write a function that returns the largest sum
/// of non-adjacent numbers. Numbers can be `0` or negative.
///
/// For example, `[2, 4, 6, 2, 5]` should return `13`, since we pick `2`, `6`,
/// and `5`. `[5, 1, 1, 5]` should return `10`, since we pick `5` and `5`.
///
/// Follow-up: Can you do this in O(N) time and constant space?
pub fn solve(data: &[i32]) -> usize {
    const ZERO: &i32 = &0;

    let len = data.len();
    let mut sum = 0;
    let mut i = 0;

    while i < len {
        let a = *data.get(i).unwrap();
        if a < 1 {
            i += 1;
            continue;
        }

        let b = *data.get(i + 1).unwrap_or(ZERO);
        if b < 1 {
            sum += a as usize;
            i += 2;

            continue;
        }

        let c = *data.get(i + 2).unwrap_or(ZERO);
        if c < 1 {
            if a >= b {
                sum += a as usize;
            } else {
                sum += b as usize;
            }

            i += 3;
            continue;
        }

        let d = *data.get(i + 3).unwrap_or(ZERO);
        if d < 1 {
            if a + c > b {
                sum += (a + c) as usize;
            } else {
                sum += b as usize;
            }

            i += 4;
        } else if b + d > a + c {
            sum += b as usize;
            i += 3;
        } else {
            sum += a as usize;
            i += 2;
        }
    }

    sum
}
