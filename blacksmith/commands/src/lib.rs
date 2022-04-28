pub struct Blacksmithcommands {
    initialized: bool
}

impl Blacksmithcommands {
    pub fn new() -> Self {
        Self {
            initialized: true
        }
    }

    pub fn print(&mut self) {
        println!("BlackSmithMC-commands is: {}", self.initialized);
    }
}