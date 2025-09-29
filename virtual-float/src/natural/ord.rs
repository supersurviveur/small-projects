use std::cmp::Ordering;

use crate::natural::Natural;

impl std::cmp::PartialOrd for Natural {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Natural {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.chunk_count() > other.chunk_count() {
            Ordering::Greater
        } else if self.chunk_count() < other.chunk_count() {
            Ordering::Less
        } else {
            self.inner.iter().rev().cmp(other.inner.iter().rev())
        }
    }
}
