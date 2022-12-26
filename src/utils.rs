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

#[derive(Debug)]
pub struct HeapElement<K, V> {
    pub key: K,
    pub value: V,
}

impl<K: Ord, V> From<(K, V)> for HeapElement<K, V> {
    fn from(x: (K, V)) -> Self {
        HeapElement {
            key: x.0,
            value: x.1,
        }
    }
}

impl<K: Ord, V> std::cmp::PartialEq for HeapElement<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Ord, V> Eq for HeapElement<K, V> {}

impl<K: Ord, V> Ord for HeapElement<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K: Ord, V> PartialOrd for HeapElement<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
