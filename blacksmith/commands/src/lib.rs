



pub struct Command {
    pub name: String,
    pub allowed: bool,
}

impl Command {
    pub fn new(nm: &str) -> Self {
        let name = nm.to_owned();
        Self { name, allowed: true }
    }
}




pub struct Blacksmithcommands {
    pub commands: Vec<Command>,
    // initialized: bool,
}

impl Blacksmithcommands {
    // pub fn run(name: String){
    //     match self.commands.name {

    //     }
    // }
}