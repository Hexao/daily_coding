/// Given a stream of elements too large to store in memory,
/// pick a random element from the stream with uniform probability.
pub fn pick_random(stream: &[i32]) -> i32 {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let mut element = 1_usize;
    let mut result = 0;

    for item in stream {
        let proba = 1.0 / element as f64;
        let r: f64 = rng.gen();
        element += 1;

        if r <= proba {
            result = *item;
        }
    }

    result
}
