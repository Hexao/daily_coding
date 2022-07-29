/// You run an e-commerce website and want to record the last `N` `order`
/// ids in a log. Implement a data structure to accomplish this,
/// with the following API:
///
/// record(order_id): adds the order_id to the log
/// get_last(i): gets the ith last element from the log.
/// i is guaranteed to be smaller than or equal to N.
///
/// You should be as efficient with time and space as possible.
pub struct Order<const N: usize> {
    ids: [u32; N],

    begin: usize,
    len: usize,
}

impl<const N: usize> Order<N> {
    pub fn record(&mut self, order_id: u32) {
        let id = (self.begin + self.len) % N;
        self.ids[id] = order_id;

        if self.len < N {
            self.len += 1;
        } else {
            self.begin = (self.begin + 1) % N;
        }
    }

    pub fn get_last(&self, ith: usize) -> Option<u32> {
        if ith >= self.len { return None; }

        let id = (self.begin + self.len - ith - 1) % N;
        Some(self.ids[id])
    }
}

impl<const N: usize> Default for Order<N> {
    fn default() -> Self {
        Self { ids: [0; N], begin: 0, len: 0 }
    }
}
