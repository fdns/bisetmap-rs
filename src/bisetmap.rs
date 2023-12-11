use std::{
    collections::{hash_map::RandomState, HashMap},
    hash::{BuildHasher, Hash},
};

use crate::BiSetDataMap;

pub struct BiSetMap<K, V, S = RandomState>(BiSetDataMap<K, V, S, ()>);

impl<K, V, S> Default for BiSetMap<K, V, S>
where
    S: Default,
{
    #[inline]
    fn default() -> BiSetMap<K, V, S> {
        BiSetMap {
            0: BiSetDataMap::default(),
        }
    }
}

impl<K, V, S> BiSetMap<K, V, S>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
    S: BuildHasher + Default,
{
    pub fn get_left(&self, k: &K) -> Option<&HashMap<V, (), S>> {
        self.0.get_left(k)
    }

    pub fn get_right(&self, v: &V) -> Option<&HashMap<K, (), S>> {
        self.0.get_right(v)
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.0.insert(k, v, ())
    }

    pub fn remove_left(&mut self, k: &K) {
        self.0.remove_left(k)
    }

    pub fn remove_right(&mut self, v: &V) {
        self.0.remove_right(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn left_side() {
        let mut bivecmap = BiSetMap::default();
        bivecmap.insert(1, 10);
        bivecmap.insert(1, 11);
        bivecmap.insert(2, 20);
        bivecmap.insert(2, 21);

        assert_eq!(
            bivecmap.get_left(&1).unwrap(),
            &HashMap::from([(10, ()), (11, ())])
        );
        assert_eq!(
            bivecmap.get_left(&2).unwrap(),
            &HashMap::from([(20, ()), (21, ())])
        );
    }

    #[test]
    fn right_side() {
        let mut bivecmap = BiSetMap::default();
        bivecmap.insert(10, 1);
        bivecmap.insert(11, 1);
        bivecmap.insert(20, 2);
        bivecmap.insert(21, 2);

        assert_eq!(
            bivecmap.get_right(&1).unwrap(),
            &HashMap::from([(10, ()), (11, ())])
        );
        assert_eq!(
            bivecmap.get_right(&2).unwrap(),
            &HashMap::from([(20, ()), (21, ())])
        );
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
            &HashMap::from([(1, ()), (2, ()), (10, ()), (11, ())])
        );
        assert_eq!(bivecmap.get_left(&2), None);
        assert_eq!(bivecmap.get_right(&1).unwrap(), &HashMap::from([(1, ())]));
        assert_eq!(bivecmap.get_right(&2).unwrap(), &HashMap::from([(1, ())]));
        assert_eq!(bivecmap.get_right(&10).unwrap(), &HashMap::from([(1, ())]));
        assert_eq!(bivecmap.get_right(&11).unwrap(), &HashMap::from([(1, ())]));
        assert_eq!(bivecmap.get_right(&20), None);
        assert_eq!(bivecmap.get_right(&21), None);
    }

    #[test]
    fn non_existing_remove_left() {
        let mut bivecmap = BiSetMap::default();
        bivecmap.insert(1, 10);
        bivecmap.remove_left(&10);

        assert_eq!(bivecmap.get_left(&1).unwrap(), &HashMap::from([(10, ())]));
        assert_eq!(bivecmap.get_right(&10).unwrap(), &HashMap::from([(1, ())]));
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

        assert_eq!(
            bivecmap.get_left(&1).unwrap(),
            &HashMap::from([(1, ()), (10, ()), (11, ())])
        );
        assert_eq!(
            bivecmap.get_left(&2).unwrap(),
            &HashMap::from([(1, ()), (20, ()), (21, ())])
        );
        assert_eq!(
            bivecmap.get_right(&1).unwrap(),
            &HashMap::from([(1, ()), (2, ())])
        );
        assert_eq!(bivecmap.get_right(&2), None);
        assert_eq!(bivecmap.get_right(&10).unwrap(), &HashMap::from([(1, ())]));
        assert_eq!(bivecmap.get_right(&11).unwrap(), &HashMap::from([(1, ())]));
        assert_eq!(bivecmap.get_right(&20).unwrap(), &HashMap::from([(2, ())]));
        assert_eq!(bivecmap.get_right(&21).unwrap(), &HashMap::from([(2, ())]));
    }

    #[test]
    fn non_existing_remove_right() {
        let mut bivecmap = BiSetMap::default();
        bivecmap.insert(1, 10);
        bivecmap.remove_right(&1);

        assert_eq!(bivecmap.get_left(&1).unwrap(), &HashMap::from([(10, ())]));
        assert_eq!(bivecmap.get_right(&10).unwrap(), &HashMap::from([(1, ())]));
    }

    #[test]
    fn with_hasher() {
        let mut bivecmap = BiSetMap::<_, _, RandomState>::default();
        bivecmap.insert(1, 10);

        assert_eq!(bivecmap.get_left(&1).unwrap(), &HashMap::from([(10, ())]));
        assert_eq!(bivecmap.get_right(&10).unwrap(), &HashMap::from([(1, ())]));
    }
}
