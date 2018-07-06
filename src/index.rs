use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::fmt::Debug;
use util;

pub trait Index {
    fn insert(&mut self, index_field: util::Val, at: usize);
    fn get_pos(&self, index_field: util::Val) -> Option<&usize>;
}

#[derive(Debug, Default)]
pub struct BasicIndex {
    map: HashMap<util::Val, usize>,
}

impl BasicIndex {
    pub fn new() -> BasicIndex {
        BasicIndex {
            map: HashMap::new()
        }
    }
}

impl Index for BasicIndex {
    fn insert(&mut self, index_field: util::Val, at: usize) {
        self.map.insert(index_field, at);
    }

    fn get_pos(&self, index_field: util::Val) -> Option<&usize> {
        self.map.get(&index_field)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_index_returns_pos() {
        let mut bi: BasicIndex = BasicIndex::new();
        bi.insert(util::Val::U32(21), 40);
        bi.insert(util::Val::U32(32), 30);
        assert_eq!(Some(&40usize), bi.get_pos(util::Val::U32(21)));
        assert_eq!(Some(&30usize), bi.get_pos(util::Val::U32(32)));
    }

    #[test]
    fn test_basic_index_return_none_for_missing_key() {
        let mut bi: BasicIndex = BasicIndex::new();
        bi.insert(util::Val::U32(21), 40);
        assert_eq!(None, bi.get_pos(util::Val::U32(20)));
    }
}
