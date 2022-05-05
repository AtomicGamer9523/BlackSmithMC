use quill::{Plugin, PluginInfo, Setup};

pub struct BlackSmithExamplePlugin;

impl Plugin for BlackSmithExamplePlugin {
    type State = ();

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "BlackSmithExamplePlugin",
            id: "bsmc_example_plugin",
        }
    }

    fn initialize(&mut self, setup: &mut dyn Setup<Self>) -> anyhow::Result<Self::State> {
        println!("This plugin Works !");
        Ok(())
    }
}