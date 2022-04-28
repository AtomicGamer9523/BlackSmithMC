use blacksmithmc_commands::Blacksmithcommands;
pub struct Blacksmith {
    initialized: bool,
}

impl Blacksmith {
    pub fn new() -> Self {
        Self { initialized: true }
    }

    pub fn print(&mut self) -> String {
        let mut bsmcc = Blacksmithcommands::new();
        String::from(format!("BlackSmithMC is: {}\n{}", self.initialized, bsmcc.print()))
    }
}
