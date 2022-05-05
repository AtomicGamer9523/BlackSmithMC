# Plugins

## Default Plugins:

1. `example` -> Example function, you can use it to make your own
2. `world-format` -> A **VERY** important plugin, used for the `.bsmc` world file format

<br>

## New plugin importing:

1. Create new directory in [`plugins`](../plugins/) with a unique name
2. Paste Code there
3. Add the package in [BlackSmithMC's `Cargo.toml`](../blacksmith/Cargo.toml) under `dependencies`
3. Add the package in [Workspace's `Cargo.toml`](../Cargo.toml) under `workspace.members`
5. Import it in [`main.rs`](../blacksmith/src/main.rs) somehow like this:

```rust
use blacksmithmc_example_plugin::BlackSmithExamplePlugin;
```

6. Use `.register_plugin(PLUGIN_NAME)` to finally load the plugin (make sure it is before the `.run()`)
7. It should compile, if it doesn't make sure the plugin is working properly