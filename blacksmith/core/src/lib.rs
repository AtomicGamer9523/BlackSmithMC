pub struct Blacksmith {
    initialized: bool
}

impl Blacksmith {
    pub fn new() -> Self {
        // let bsmcc = Blacksmithcommands::new();
        // bsmcc.print();
        Self {
            initialized: true
        }
    }

    pub fn print(&mut self) -> String {
        String::from(format!("BlackSmithMC is: {}", self.initialized))
    }
}