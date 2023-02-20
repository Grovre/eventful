use std::collections::HashMap;

pub struct Actions {
    registered: HashMap<u128, fn()>,
    id: u128,
}

impl Actions {
    pub fn new() -> Self {
        Actions {
            registered: HashMap::new(),
            id: 0,
        }
    }

    pub fn add_action(&mut self, f: fn()) -> u128 {
        self.registered.insert(self.id, f);
        self.id += 1;
        self.id - 1
    }

    pub fn invoke_actions(&self) {
        self.registered.values().for_each(|action| action());
    }

    pub fn remove_action(&mut self, key: &u128) -> Option<fn()> {
        self.registered.remove(key)
    }
}
