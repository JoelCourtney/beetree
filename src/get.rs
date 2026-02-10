use crate::{Branch, Map, MaybeBox, Motion, Visitor};

struct GetVisitor<'a, K, V> {
    key: &'a K,
    result: Option<*mut V>,
}

impl<K: Ord, V> Visitor<K, V> for GetVisitor<'_, K, V> {
    #[inline]
    fn visit_internal(&mut self, array: &mut [Branch<K, V>]) -> Motion {
        match array.binary_search_by(|b| b.key.cmp(self.key)) {
            Ok(i) => {
                self.result = Some(array[i].value.boxify());
                Motion::Finish
            }
            Err(i) => Motion::Down(i),
        }
    }

    #[inline]
    fn visit_leaf(&mut self, array: &mut [(K, V)]) {
        if let Ok(i) = array.binary_search_by(|(k, _)| k.cmp(self.key)) {
            self.result = Some(&mut array[i].1);
        }
    }
}

impl<K: Ord, V> Map<K, V> {
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut visitor = GetVisitor { key, result: None };
        self.accept_visitor(&mut visitor);
        visitor.result.map(|ptr| unsafe { &*ptr })
    }
    
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut visitor = GetVisitor { key, result: None };
        self.accept_visitor(&mut visitor);
        visitor.result.map(|ptr| unsafe { &mut *ptr })
    }
}

// struct GetBeforeVisitor<'a, K, V> {
    // key: &'a K,
    // inclusive: bool,
    // previous_branch: Option<*mut Branch<K, V>>,
    // result: Option<(*const K, *mut V)>
// }
// 
// 
// impl<K: Ord, V> Visitor<K, V> for GetBeforeVisitor<'_, K, V> {
    // #[inline]
    // fn visit_internal(&mut self, array: &mut [Branch<K, V>]) -> Motion {
        // match array.binary_search_by(|b| b.key.cmp(self.key)) {
            // Ok(i) if self.inclusive => {
                // self.result = Some((array[i].array[i].value.boxify());
                // Motion::Finish
            // }
            // Err(i) => Motion::Down(i),
        // }
    // }
// 
    // #[inline]
    // fn visit_leaf(&mut self, array: &mut [(K, V)]) {
        // if let Ok(i) = array.binary_search_by(|(k, _)| k.cmp(self.key)) {
            // self.result = Some(&mut array[i].1);
        // }
    // }
// }

#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;

    use crate::{B, Map};

    #[test]
    fn get_from_empty() {
        let map: Map<usize, usize> = Map::new();
        assert_eq!(map.get(&5), None);
    }

    #[test]
    fn get_one() {
        let mut map = Map::new();
        map.insert(2, 3);

        assert_eq!(map.get(&2), Some(&3));
    }

    #[test]
    fn insert_ordered() {
        let mut map = Map::new();
        for i in 10..15 {
            map.insert(i, i * 2);
        }

        assert_eq!(map.get(&12), Some(&24));
        assert_eq!(map.get(&14), Some(&28));
        assert_eq!(map.get(&16), None);
        assert_eq!(map.get(&7), None);
    }

    #[test]
    fn insert_ordered_overflow() {
        let mut map = Map::new();
        for i in 10..10 + B * 3 {
            map.insert(i, i * 2);
        }

        let index: Vec<_> = (10..10 + B * 3).collect();
        for i in index {
            assert_eq!(map.get(&i), Some(&(i * 2)));
        }

        assert_eq!(map.get(&7), None);
        assert_eq!(map.get(&(15 + B * 3)), None);
    }

    #[test]
    fn insert_ordered_overflow_get_random() {
        let mut map = Map::new();
        for i in 10..10 + B * 3 {
            map.insert(i, i * 2);
        }

        let mut index: Vec<_> = (10..10 + B * 3).collect();
        index.shuffle(&mut rand::rng());
        for i in index {
            assert_eq!(map.get(&i), Some(&(i * 2)));
        }

        assert_eq!(map.get(&7), None);
        assert_eq!(map.get(&(15 + B * 3)), None);
    }

    #[test]
    fn get_head_of_branch() {
        let mut map = Map::new();
        for i in 0..(B * 3) {
            map.insert(i, i * 2);
        }

        assert_eq!(map.get(&(B / 2)), Some(&B));
    }

    #[test]
    fn get_in_new_branch() {
        let mut map = Map::new();
        for i in 0..(B * 3) {
            map.insert(i, i * 2);
        }

        assert_eq!(map.get(&(B / 2 * 3)), Some(&(B * 3)));
    }
    
    #[test]
    fn get_mut() {
        let mut map = Map::new();
        map.insert(1, 1);
        *map.get_mut(&1).unwrap() = 5;
        assert_eq!(map.get(&1), Some(&5));
    }
}
