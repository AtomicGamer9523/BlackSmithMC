use quill::{Plugin, PluginInfo, Setup, PluginLogger};

pub struct BlackSmithExamplePlugin;

impl Plugin for BlackSmithExamplePlugin {
    type State = ();

    fn info(&self) -> PluginInfo {
        PluginInfo {
            //? Shouldn't repeat itself (NO SPACES)
            name: "BlackSmithExamplePlugin",
            //? NO REPEAT, NO SPACES, UNIQUE
            id: "bsmc_example_plugin",
        }
    }

    fn debug(&self){
        PluginLogger::debug(self.info().name, "Working ;)".to_string());
    }

    fn initialize(&mut self, setup: &mut dyn Setup<Self>) -> anyhow::Result<Self::State> {
        PluginLogger::info(self.info().name, "I have been Initialized".to_string());
        Ok(())
    }
}