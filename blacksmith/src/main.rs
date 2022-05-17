use plugins::{
    website_plugin::WebsitePlugin,
    console_plugin::ConsolePlugin,
    //? Uncomment bellow to load example plugin
    //blacksmithmc_example_plugin::BlackSmithExamplePlugin,
};
use feather_server::ServerBuilder;


fn main() -> anyhow::Result<()> {
    ServerBuilder::new()?
    .register_default_plugins()
    .register_plugin(WebsitePlugin)
    .register_plugin(ConsolePlugin)
    //? Uncomment bellow to register example plugin
    //.register_plugin(BlackSmithExamplePlugin)


    
    .run()
}