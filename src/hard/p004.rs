/// Given an array of integers, find the first missing positive integer
/// in **linear time** and **constant space**. In other words, find the
/// lowest positive integer that does not exist in the array. The array
/// can contain duplicates and negative numbers as well.
///
/// For example, the input `[3, 4, -1, 1]` should give `2`.
/// The input `[1, 2, 0]` should give `3`.
///
/// You can modify the input array in-place.
pub fn solve(mut data: Vec<i32>) -> i32 {
    let end = sort_pos_neg(&mut data);

    for i in 0..end {
        let item = data[i].abs() as usize - 1;

        if item < end && data[item] > 0 {
            data[item] = -data[item];
        }
    }

    data.iter().take(end)
        .take_while(|&&item| item < 0)
        .count() as i32 + 1
}

/// Sort the `data` vector and places all negativ numbers at the end.
fn sort_pos_neg(data: &mut Vec<i32>) -> usize {
    let mut i = data.len();
    if i == 0 { return 0; }

    let mut j = i - 1;

    loop {
        i -= 1;

        if data[i] <= 0 {
            let tmp = data[i];
            data[i] = data[j];
            data[j] = tmp;
            j -= 1;
        }

        if i == 0 { break; }
    }

    j + 1
}
