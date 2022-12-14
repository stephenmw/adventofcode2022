use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn top_n<I, T>(iter: I, n: usize) -> impl std::iter::Iterator<Item = T>
where
    I: std::iter::IntoIterator<Item = T>,
    T: Ord,
{
    let mut heap = BinaryHeap::with_capacity(n);
    for v in iter.into_iter() {
        if heap.len() < n {
            heap.push(Reverse(v));
        } else {
            heap.peek_mut().map(|mut x| {
                if v > x.0 {
                    *x = Reverse(v)
                }
            });
        }
    }

    heap.into_iter().map(|x| x.0).into_iter()
}
