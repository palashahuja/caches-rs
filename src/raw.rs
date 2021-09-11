mod core;
mod iterators;
mod ll;

pub use self::core::{OnEvictCallback, RawLRU};
pub use self::ll::KeyRef;

#[cfg(feature = "nightly")]
pub use self::ll::NotKeyRef;

pub use iterators::{
    IntoIter, Iter, IterMut, Keys, ReversedIter, ReversedIterMut, ReversedKeys, ReversedValues,
    ReversedValuesMut, Values, ValuesMut,
};

/// An intermediate trait for specialization of `Extend`.
#[doc(hidden)]
trait SpecExtend<I: IntoIterator> {
    /// Extends `self` with the contents of the given iterator.
    fn spec_extend(&mut self, iter: I);
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::string::ToString;
    #[cfg(feature = "hashbrown")]
    use hashbrown::{HashMap, HashSet};
    use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
    #[cfg(not(feature = "hashbrown"))]
    use std::collections::{HashMap, HashSet};
    use std::vec::Vec;

    #[test]
    fn test_simple_lru_copy_key_value() {
        let mut cache = RawLRU::new(5);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3);
        cache.put(4, 4);
        cache.put(5, 5);
        cache.put(6, 6);

        assert_eq!(cache.len(), 5);

        assert_eq!(None, cache.get(&1));

        let v = cache.get_mut(&2).unwrap();
        *v = 5;
        assert_eq!(5, *cache.get(&2).unwrap());

        let v = cache.get(&3).unwrap();
        assert_eq!(3, *v);

        let v = cache.get(&4).unwrap();
        assert_eq!(4, *v);

        let v = cache.get(&5).unwrap();
        assert_eq!(5, *v);

        let v = cache.get(&6).unwrap();
        assert_eq!(6, *v);

        let oldest = cache.peek_oldest().unwrap();
        assert_eq!(oldest, (&2, &5));
        let oldest = cache.remove_oldest().unwrap();
        assert_eq!(oldest, (2, 5));

        assert_eq!(cache.len(), 4);

        let v = cache.remove(&3).unwrap();
        assert_eq!(v, 3);
        assert_eq!(cache.len(), 3);

        let v = cache.peek(&4).unwrap();
        assert_eq!(*v, 4);

        let v = cache.peek_mut(&4).unwrap();
        *v = 2;
        assert_eq!(cache.get(&4), Some(&2));
    }

    #[test]
    fn test_simple_lru_copy_key() {
        let mut cache = RawLRU::new(3);
        cache.put(1, "a".to_string());
        cache.put(2, "b".to_string());
        cache.put(3, "c".to_string());

        let v = cache.peek(&1).unwrap();
        assert_eq!(*v, "a".to_string());

        let v = cache.peek_mut(&1).unwrap();
        *v = "aa".to_string();

        let v = cache.peek_oldest_mut().unwrap();
        *v.1 = "aaa".to_string();

        let v = cache.peek_oldest().unwrap();
        assert_eq!(*v.1, "aaa".to_string());

        let v = cache.get(&2).unwrap();
        assert_eq!(*v, "b".to_string());

        let v = cache.get_mut(&2).unwrap();
        *v = "bb".to_string();
    }

    #[test]
    fn test_from_maps() {
        let mut map = HashMap::new();
        map.insert(1, 0);
        map.insert(2, 0);
        map.insert(3, 0);
        let cache = RawLRU::from(map);
        cache.iter().for_each(|(k, v)| {
            assert!(cache.contains(k));
        });

        let mut map = BTreeMap::new();
        map.insert(1, 0);
        map.insert(2, 0);
        map.insert(3, 0);
        let cache = RawLRU::from(map);
        cache.iter().for_each(|(k, v)| {
            assert!(cache.contains(k));
        });
    }

    #[test]
    fn test_from_iters() {
        let mut iter = Vec::new();
        iter.push((1, 0));
        iter.push((2, 0));
        iter.push((3, 0));
        let cache = RawLRU::from(iter);
        cache.iter().for_each(|(k, v)| {
            assert!(cache.contains(k));
        });

        let cache = RawLRU::from(&*[(1, 0), (2, 0), (3, 0)].to_vec());
        cache.iter().for_each(|(k, v)| {
            assert!(cache.contains(k));
        });

        let mut iter = HashSet::new();
        iter.insert((1, 0));
        iter.insert((2, 0));
        iter.insert((3, 0));
        let cache = RawLRU::from(iter);
        cache.iter().for_each(|(k, v)| {
            assert!(cache.contains(k));
        });

        let mut iter = BTreeSet::new();
        iter.insert((1, 0));
        iter.insert((2, 0));
        iter.insert((3, 0));
        let cache = RawLRU::from(iter);
        cache.iter().for_each(|(k, v)| {
            assert!(cache.contains(k));
        });
    }

    #[test]
    fn test_raw_lru_drop() {
        let mut cache = RawLRU::new(3);
        cache.put(1, "a".to_string());
        cache.put(2, "b".to_string());
        cache.put(3, "c".to_string());
        drop(cache);
    }
}
