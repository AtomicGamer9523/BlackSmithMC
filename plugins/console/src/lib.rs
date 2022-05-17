use quill::{Plugin, PluginInfo, Setup, PluginLogger};
// use vane::{Entities, EntityBuilder, Component};
use std::thread;
use std::io::{self, BufRead};




pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    type State = ();

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "ConsolePlugin",
            id: "console",
        }
    }

    fn debug(&self){
        PluginLogger::debug(self.info().name, "Console Initialized".to_string());
    }

    fn initialize(&mut self, setup: &mut dyn Setup<Self>) -> anyhow::Result<Self::State> {
        thread::spawn(move || {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let data: String = line.unwrap();
                println!("{}", data);
            }
        });

        Ok(())
    }
}
