use std::collections::HashMap;

pub struct Prefab {
    pub name: &'static str,
}

impl Prefab {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

pub struct PrefabEntityTracker(pub HashMap<u64, bool>);
