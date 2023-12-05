use crate::almanac::AlmanacItem;

pub struct RangeMap<K, V> {
    ranges: Vec<(K, V, u64)>,
}

impl<K: AlmanacItem, V: AlmanacItem> RangeMap<K, V> {
    pub fn new(ranges: impl IntoIterator<Item = (V, K, u64)>) -> Self {
        let mut ranges = ranges
            .into_iter()
            .map(|(v, k, len)| (k, v, len))
            .collect::<Vec<_>>();

        ranges.sort_by_key(|(k, _, _)| *k);

        Self { ranges }
    }

    pub fn get(&self, key: K) -> V {
        let value = self.ranges
            .iter()
            .find(|(k, _, len)| *k <= key && key.value() < k.value() + len)
            .map(|(k, v, _)| v.value() + key.value() - k.value())
            .unwrap_or(key.value());

        V::new(value)
    }
}
