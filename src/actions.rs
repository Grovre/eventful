use std::{collections::HashMap, fmt::Display, str::FromStr};

pub struct Actions {
    registered: HashMap<u128, Box<dyn Fn()>>,
    next_id: u128,
}

impl Actions {
    pub fn new() -> Self {
        Actions {
            registered: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_action(&mut self, f: Box<dyn Fn()>) -> u128 {
        self.registered.insert(self.next_id, f);
        self.next_id += 1;
        self.next_id - 1
    }

    pub fn invoke_actions(&self) {
        self.registered.values().for_each(|action| action());
    }

    pub fn remove_action(&mut self, key: &u128) -> Option<Box<dyn Fn()>> {
        self.registered.remove(key)
    }
}

impl Display for Actions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from_str("Key -> Action:").unwrap();

        if self.registered.len() == 0 {
            str.push_str("\nEmpty");
            return write!(f, "{}\nEmpty", str);
        }

        for entry in &self.registered {
            let map_str = format!("\n{} -> {:p}", entry.0, *entry.1);
            str.push_str(map_str.as_str());
        }

        write!(f, "{}", str)
    }
}
