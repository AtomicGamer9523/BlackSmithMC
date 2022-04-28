pub struct Blacksmithcommands {
    initialized: bool,
}

impl Blacksmithcommands {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn print(&mut self) -> String {
        String::from(format!("BlackSmithMC-commands is: {}", self.initialized))
    }
}
