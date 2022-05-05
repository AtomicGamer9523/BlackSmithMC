use feather_server::ServerBuilder;
//? Uncomment bellow to load example plugin
//use blacksmithmc_example_plugin::BlackSmithExamplePlugin;

fn main() -> anyhow::Result<()> {
    ServerBuilder::new()?
    .register_default_plugins()
    //? Uncomment bellow to register example plugin
    //.register_plugin(BlackSmithExamplePlugin)


    .run()
}