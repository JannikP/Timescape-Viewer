use dashmap::{DashMap, iter::Iter, mapref::one::Ref};
use std::{
    hash::RandomState,
    sync::atomic::{AtomicUsize, Ordering},
};

use super::{Signal, SignalId};

#[derive(Debug)]
pub struct Vocabulary {
    storage: DashMap<SignalId, Signal>,
    next_id: AtomicUsize,
}

impl Vocabulary {
    pub fn new() -> Self {
        Vocabulary {
            storage: DashMap::new(),
            next_id: AtomicUsize::new(0),
        }
    }

    fn add(&self, signal: Signal) -> SignalId {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.storage.insert(id, signal);
        id
    }

    fn find<'a>(&'a self, name: &str) -> Option<Ref<'a, SignalId, Signal>> {
        None
    }

    fn get<'a>(&'a self, id: SignalId) -> Option<Ref<'a, SignalId, Signal>> {
        self.storage.get(&id)
    }

    fn signals<'a>(
        &'a self,
    ) -> Iter<'a, SignalId, Signal, RandomState, DashMap<SignalId, Signal, RandomState>> {
        self.storage.iter()
    }
}
