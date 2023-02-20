pub struct Actions {
    registered: Vec<fn()>,
}

impl Actions {
    pub fn new() -> Self {
        Actions {
            registered: Vec::new(),
        }
    }

    pub fn add_action(&mut self, f: fn()) {
        self.registered.push(f);
    }

    pub fn invoke_actions(&self) {
        for action in &self.registered {
            action();
        }
    }
}
