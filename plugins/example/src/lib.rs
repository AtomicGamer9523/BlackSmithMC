use quill::{Plugin, PluginInfo, Setup};

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
        quill::plog(self.info().name, "Working ;)");
    }

    fn initialize(&mut self, setup: &mut dyn Setup<Self>) -> anyhow::Result<Self::State> {
        Ok(())
    }
}