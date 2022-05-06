use feather_server::ServerBuilder;
use website_plugin::WebsitePlugin;
//? Uncomment bellow to load example plugin
//use blacksmithmc_example_plugin::BlackSmithExamplePlugin;


fn main() -> anyhow::Result<()> {
    ServerBuilder::new()?
    .register_default_plugins()
    .register_plugin(WebsitePlugin)
    //? Uncomment bellow to register example plugin
    //.register_plugin(BlackSmithExamplePlugin)


    .run()
}