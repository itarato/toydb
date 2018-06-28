use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::fmt::Debug;

pub trait Index<T: Eq + Hash> {
    fn insert(&mut self, index_field: T, at: usize);
    fn get_pos(&self, index_field: T) -> Option<&usize>;
}

#[derive(Debug, Default)]
pub struct BasicIndex<T: Eq + Hash + Debug> {
    map: HashMap<T, usize>,
}

impl<T: Eq + Hash + Debug> BasicIndex<T> {
    pub fn new() -> BasicIndex<T> {
        BasicIndex {
            map: HashMap::new()
        }
    }
}

impl<T: Eq + Hash + Debug> Index<T> for BasicIndex<T> {
    fn insert(&mut self, index_field: T, at: usize) {
        self.map.insert(index_field, at);
    }

    fn get_pos(&self, index_field: T) -> Option<&usize> {
        self.map.get(&index_field)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_index_returns_pos() {
        let mut bi: BasicIndex<i32> = BasicIndex::new();
        bi.insert(-21, 40);
        bi.insert(32, 30);
        assert_eq!(Some(&40usize), bi.get_pos(-21));
        assert_eq!(Some(&30usize), bi.get_pos(32));
    }

    #[test]
    fn test_basic_index_return_none_for_missing_key() {
        let mut bi: BasicIndex<i32> = BasicIndex::new();
        bi.insert(-21, 40);
        assert_eq!(None, bi.get_pos(-20));
    }
}
