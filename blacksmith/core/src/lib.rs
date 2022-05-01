pub struct Blacksmith {
    initialized: bool,
}

impl Blacksmith {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn status(&self) -> bool {
        self.initialized
    }
}
