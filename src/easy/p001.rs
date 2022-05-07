/// Given a list of numbers and a number `k`, return whether
/// any two numbers from the list add up to `k`
/// 
/// Bonus: Can you do this in one pass?
pub fn solve(data: &[i32], target: i32) -> bool {
    data.iter().enumerate()
        .flat_map(|(id, element)| {
            (&data[id + 1..]).iter().map(move |item| item + element)
        })
        .any(|item| item == target)
}
