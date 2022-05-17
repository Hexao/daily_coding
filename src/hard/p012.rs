/// There exist a staircase with `N` steps, and you can climb up either
/// `1` or `2` steps at a time. Given `N`, write a function that returns
/// the number of unique ways you can climb the staircase.
/// The order of the steps matters.
///
/// For exemple, if `N` is `4`, then there are `5` unique ways:
/// * 1, 1, 1, 1
/// * 2, 1, 1
/// * 1, 2, 1
/// * 1, 1, 2
/// * 2, 2
///
/// What if, instead of behing able to climb `1` or `2` steps at a time, you
/// could climb any numger from a set of positive integer `X`? For example,
/// if `X = {1, 3, 5}`, you could climb `1`, `3`, or `5` steps at a time.
pub fn solve_extended(staircase_len: u32, set: &[u32]) -> usize {
    let mut log = Vec::with_capacity(staircase_len as usize);
    let mut acc = 0;

    for i in set {
        log.push(*i);

        let (inner, ret) = solve(staircase_len, set, *i, log);
        acc += inner;
        log = ret;

        log.pop();
    }

    acc
}

pub fn solve_basic(staircase_len: u32) -> usize {
    let mut log = Vec::with_capacity(staircase_len as usize);
    let set = &[1, 2];
    let mut acc = 0;

    for i in set {
        log.push(*i);

        let (inner, ret) = solve(staircase_len, set, *i, log);
        acc += inner;
        log = ret;

        log.pop();
    }

    acc
}

fn solve(staircase_len: u32, set: &[u32], dist: u32, mut log: Vec<u32>) -> (usize, Vec<u32>) {
    let mut count = 0;

    for i in set {
        let dist = dist + i;
        log.push(*i);

        match staircase_len.cmp(&dist) {
            std::cmp::Ordering::Greater => {
                let (inner, ret) = solve(staircase_len, set, dist, log);
                count += inner;
                log = ret;
            }
            std::cmp::Ordering::Equal => {
                println!("{log:?}");
                count += 1;
            }
            std::cmp::Ordering::Less => {}
        }

        log.pop();
    }

    (count, log)
}
