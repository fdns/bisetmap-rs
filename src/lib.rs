use std::{
    collections::{hash_map::RandomState, HashMap, HashSet},
    hash::{BuildHasher, Hash},
};

#[derive(Debug, Clone)]
pub struct BiSetMap<K, V, S = RandomState> {
    left: HashMap<K, HashSet<V>, S>,
    right: HashMap<V, HashSet<K>, S>,
}

impl<K, V> Default for BiSetMap<K, V, RandomState> {
    #[inline]
    fn default() -> BiSetMap<K, V, RandomState> {
        BiSetMap {
            left: HashMap::with_hasher(Default::default()),
            right: HashMap::with_hasher(Default::default()),
        }
    }
}

impl<K, V, S> BiSetMap<K, V, S>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
    S: BuildHasher,
{
    pub fn get_left(&self, k: &K) -> Option<&HashSet<V>> {
        self.left.get(k)
    }

    pub fn get_right(&self, v: &V) -> Option<&HashSet<K>> {
        self.right.get(v)
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.left.entry(k.clone()).or_default().insert(v.clone());
        self.right.entry(v).or_default().insert(k);
    }

    pub fn remove_left(&mut self, k: &K) {
        Self::remove(&mut self.left, &mut self.right, k);
    }

    pub fn remove_right(&mut self, v: &V) {
        Self::remove(&mut self.right, &mut self.left, v);
    }

    fn remove<A: Eq + Hash + Clone, B: Eq + Hash + Clone>(
        left_map: &mut HashMap<A, HashSet<B>, S>,
        right_map: &mut HashMap<B, HashSet<A>, S>,
        k: &A,
    ) {
        let left = left_map.remove(k);
        if let Some(left) = left {
            for right in left {
                let elem = right_map.get_mut(&right).unwrap();
                elem.remove(k);
                if elem.is_empty() {
                    right_map.remove(&right);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::BiSetMap;

    #[test]
    fn left_side() {
        let mut bivecmap = BiSetMap::default();
        bivecmap.insert(1, 10);
        bivecmap.insert(1, 11);
        bivecmap.insert(2, 20);
        bivecmap.insert(2, 21);

        assert_eq!(bivecmap.get_left(&1).unwrap(), &HashSet::from([10, 11]));
        assert_eq!(bivecmap.get_left(&2).unwrap(), &HashSet::from([20, 21]));
    }

    #[test]
    fn right_side() {
        let mut bivecmap = BiSetMap::default();
        bivecmap.insert(10, 1);
        bivecmap.insert(11, 1);
        bivecmap.insert(20, 2);
        bivecmap.insert(21, 2);

        assert_eq!(bivecmap.get_right(&1).unwrap(), &HashSet::from([10, 11]));
        assert_eq!(bivecmap.get_right(&2).unwrap(), &HashSet::from([20, 21]));
    }

    #[test]
    fn partial_remove_left() {
        let mut bivecmap = BiSetMap::default();
        bivecmap.insert(1, 1);
        bivecmap.insert(1, 2);
        //bivecmap.insert(2, 1);
        //bivecmap.insert(2, 2);
        bivecmap.insert(1, 10);
        bivecmap.insert(1, 11);
        bivecmap.insert(2, 20);
        bivecmap.insert(2, 21);

        bivecmap.remove_left(&2);

        assert_eq!(
            bivecmap.get_left(&1).unwrap(),
            &HashSet::from([1, 2, 10, 11])
        );
        assert_eq!(bivecmap.get_left(&2), None);
        assert_eq!(bivecmap.get_right(&1).unwrap(), &HashSet::from([1]));
        assert_eq!(bivecmap.get_right(&2).unwrap(), &HashSet::from([1]));
        assert_eq!(bivecmap.get_right(&10).unwrap(), &HashSet::from([1]));
        assert_eq!(bivecmap.get_right(&11).unwrap(), &HashSet::from([1]));
        assert_eq!(bivecmap.get_right(&20), None);
        assert_eq!(bivecmap.get_right(&21), None);

        // Check that memory is free
        assert_eq!(bivecmap.left.get(&2), None);
        assert_eq!(bivecmap.right.get(&20), None);
        assert_eq!(bivecmap.right.get(&21), None);
    }

    #[test]
    fn fully_remove_left() {
        let mut bivecmap = BiSetMap::default();
        bivecmap.insert(1, 10);
        bivecmap.remove_left(&1);

        // Check maps are empty
        assert!(bivecmap.left.is_empty());
        assert!(bivecmap.right.is_empty());
    }

    #[test]
    fn non_existing_remove_left() {
        let mut bivecmap = BiSetMap::default();
        bivecmap.insert(1, 10);
        bivecmap.remove_left(&10);

        assert_eq!(bivecmap.get_left(&1).unwrap(), &HashSet::from([10]));
        assert_eq!(bivecmap.get_right(&10).unwrap(), &HashSet::from([1]));
    }

    #[test]
    fn partial_remove_right() {
        let mut bivecmap = BiSetMap::default();
        bivecmap.insert(1, 1);
        bivecmap.insert(1, 2);
        bivecmap.insert(2, 1);
        bivecmap.insert(2, 2);
        bivecmap.insert(1, 10);
        bivecmap.insert(1, 11);
        bivecmap.insert(2, 20);
        bivecmap.insert(2, 21);

        bivecmap.remove_right(&2);

        assert_eq!(bivecmap.get_left(&1).unwrap(), &HashSet::from([1, 10, 11]));
        assert_eq!(bivecmap.get_left(&2).unwrap(), &HashSet::from([1, 20, 21]));
        assert_eq!(bivecmap.get_right(&1).unwrap(), &HashSet::from([1, 2]));
        assert_eq!(bivecmap.get_right(&2), None);
        assert_eq!(bivecmap.get_right(&10).unwrap(), &HashSet::from([1]));
        assert_eq!(bivecmap.get_right(&11).unwrap(), &HashSet::from([1]));
        assert_eq!(bivecmap.get_right(&20).unwrap(), &HashSet::from([2]));
        assert_eq!(bivecmap.get_right(&21).unwrap(), &HashSet::from([2]));

        // Check that memory is free
        assert_eq!(bivecmap.right.get(&2), None);
    }

    #[test]
    fn fully_remove_right() {
        let mut bivecmap = BiSetMap::default();
        bivecmap.insert(1, 10);
        bivecmap.remove_right(&10);

        // Check maps are empty
        assert!(bivecmap.left.is_empty());
        assert!(bivecmap.right.is_empty());
    }

    #[test]
    fn non_existing_remove_right() {
        let mut bivecmap = BiSetMap::default();
        bivecmap.insert(1, 10);
        bivecmap.remove_right(&1);

        assert_eq!(bivecmap.get_left(&1).unwrap(), &HashSet::from([10]));
        assert_eq!(bivecmap.get_right(&10).unwrap(), &HashSet::from([1]));
    }
}
