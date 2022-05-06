# BlackSmithMC

[![Download](https://img.shields.io/github/languages/code-size/AtomicGamer9523/BlackSmithMC?label=Download%20Code&logo=DocuSign&logoColor=blue)](https://codeload.github.com/AtomicGamer9523/BlackSmithMC/zip/refs/heads/master)
[![Download](https://img.shields.io/badge/Download%20Build-6.84%20MB-blue?logo=DocuSign&logoColor=blue)](https://atomicgamer9523.github.io/BlackSmithMC/build/)
[![Rust](https://github.com/AtomicGamer9523/BlackSmithMC/actions/workflows/rust.yml/badge.svg)](https://github.com/AtomicGamer9523/BlackSmithMC/actions/workflows/rust.yml)
[![pages-build-deployment](https://github.com/AtomicGamer9523/BlackSmithMC/actions/workflows/pages/pages-build-deployment/badge.svg)](https://github.com/AtomicGamer9523/BlackSmithMC/actions/workflows/pages/pages-build-deployment)
[![rust-clippy analyze](https://github.com/AtomicGamer9523/BlackSmithMC/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/AtomicGamer9523/BlackSmithMC/actions/workflows/rust-clippy.yml)

A Minecraft server implementation written in Rust.

### Status

The project is in an **early stage**. Many, many features are unimplemented. We welcome help from anyone willing to contribute!\
Documentation avaliable [here](./docs/README.md)

### Supported Minecraft versions

BlackSmithMC supports 1.18.1 clients. We do not currently have plans to support multiple versions at once, but
we may consider this in the future.

### Goals

The BlackSmithMC project aims to provide a Minecraft server that is _fast_, _modular_, and paired with an ergonomic plugin API.

Our mid-term goal is to make BlackSmithMC usable on hub and minigame servers. The limited set of gameplay features available in BlackSmithMC is not a problem for such servers that require a small subset of vanilla functionality. On the other hand, BlackSmithMC's modularity and performance lends itself to these types of servers. Therefore, our current focus is
on building a rich plugin API to enable these use cases.

In the long term, BlackSmithMC could be used on larger, more survival-like servers, where its performance should allow many players to simultaneously play on the same world requiring very few resources.

### Ecosystem

The BlackSmithMC ecosystem consists of several repositories:

* [`libcraft`](https://github.com/feather-rs/feather/tree/main/libcraft), a set of Rust crates providing Minecraft functionality.
* [`quill`](https://github.com/feather-rs/feather/tree/main/quill), our work-in-progress plugin API. Quill plugins are written in Rust and compiled to WebAssembly. BlackSmithMC runs them in a sandboxed WebAssembly VM.
* [`feather`](https://github.com/feather-rs/feather/tree/main/feather), the so called "heart" of the project
* [`BlackSmithMC`](./blacksmith/), the server software built on top of all of them.

### Performance

Comparisons to vanilla performance _will_ be extremely misleading, because BlackSmithMC implements so few features. But if you really want them:

* BlackSmithMC can handle 1,000,000 entities spawned by a plugin before it starts to max out the CPU. The vanilla server will croak long before then.
* BlackSmithMC can handle 500 concurrent player connections with each player walking in a random direction.

These results _will_ change after more features are implemented in BlackSmithMC, so take them with a grain of salt.

Memory usage in BlackSmithMC is proportional to the number of loaded chunks, not player counts. In the 500 player test, the server uses ~40 MiB of RAM
until the players start to spread out. In the 1,000,000 entities test, it uses 400 MiB of RAM without any chunks loaded.

### Running

We offer precompiled binaries for Windows, Linux, and macOS at [build](./build/).
NB: Do **NOT** use github releases, they are majorly outdated

To run BlackSmithMC:

* Extract the downloaded archive.
* Run the binary.
  * On Linux and macOS: `./server` in the server directory
  * On Windows: double-click `server.exe`
  
The server will create a configuration file (`config.toml`) which you can modify.

BlackSmithMC will generate a world by default. If you want to load a vanilla world,
copy the world save to the server directory under the name "world" (by default).

Warning: BlackSmithMC world persistence is fairly new and will likely cause problems
when attempting to open BlackSmithMC worlds in vanilla. Do not let BlackSmithMC touch worlds
you care about unless they have been backed up.

### Compiling

If you are on another platform, compile the server yourself to try it out:

```bash
git clone https://github.com/BlackSmithMC-rs/BlackSmithMC
cd BlackSmithMC
cargo build --release
```

Compiling from source requires the latest stable version of Rust. Older Rust versions may be able
to compile BlackSmithMC, but they are not guaranteed to keep working.

The server executable will be located in `target/release`.

### Architecture

For contributors, we have a work-in-progress explanation of BlackSmithMC's architecture [here](docs/architecture.md).

### FAQ

* Is BlackSmithMC production ready?

Not yet. There are numerous bugs and missing features which have yet to be resolved,
and the codebase has not been tested enough to consider the server production ready.
