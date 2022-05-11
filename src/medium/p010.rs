/// Implement a job scheduler which takes in a function `f` and
/// an integer `n`, and calls `f` after `n` milliseconds.
pub fn call_after<F>(f: F, time_out: u64) where F: FnOnce() {
    std::thread::sleep(std::time::Duration::from_millis(time_out));
    f();
}
