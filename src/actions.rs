use std::{collections::HashMap, fmt::Display, str::FromStr};

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

impl Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from_str("Action Keys:").unwrap();
        for entry in &self.registered {
            let map_str = format!("\n{} -> {:p}", entry.0, entry.1);
            str.push_str(map_str.as_str());
        }

        write!(f, "{}", str)
    }
}
