use std::collections::HashMap;
use util;

pub trait Index {
    fn insert(&mut self, val: util::Val, at: usize);
    fn get_pos(&self, val: util::Val) -> Option<&Vec<usize>>;
}

#[derive(Debug, Default)]
pub struct BasicIndex {
    map: HashMap<util::Val, Vec<usize>>,
}

impl Index for BasicIndex {
    fn insert(&mut self, val: util::Val, at: usize) {
        if self.map.contains_key(&val) {
            self.map.get_mut(&val).unwrap().push(at);
        } else {
            self.map.insert(val, vec![at]);
        }
    }

    fn get_pos(&self, index_field: util::Val) -> Option<&Vec<usize>> { self.map.get(&index_field) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_index_returns_pos() {
        let mut bi: BasicIndex = Default::default();
        bi.insert(util::Val::U32(21), 40);
        bi.insert(util::Val::U32(32), 30);
        assert_eq!(Some(&vec![40usize]), bi.get_pos(util::Val::U32(21)));
        assert_eq!(Some(&vec![30usize]), bi.get_pos(util::Val::U32(32)));
    }

    #[test]
    fn test_basic_index_return_none_for_missing_key() {
        let mut bi: BasicIndex = Default::default();
        bi.insert(util::Val::U32(21), 40);
        assert_eq!(None, bi.get_pos(util::Val::U32(20)));
    }

    #[test]
    fn test_basic_index_return_more_than_one_position() {

        let mut bi: BasicIndex = Default::default();
        bi.insert(util::Val::U32(21), 40);
        bi.insert(util::Val::U32(21), 30);
        assert_eq!(Some(&vec![40usize, 30usize]), bi.get_pos(util::Val::U32(21)));
    }
}
