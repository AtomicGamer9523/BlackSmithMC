use time::macros::format_description;
use time::OffsetDateTime;
pub mod chat;
mod chunk_lock;
pub mod components;
/// Marker components for each specific entity.
pub mod entities;
pub mod events;
pub mod game;
mod plugin;
pub mod saveload;
pub mod threadpool;
pub mod world;

#[doc(inline)]
pub use vane::{Component, Entities, Entity, EntityBuilder, EntityRef, Resources, SysResult};

#[doc(inline)]
pub use chat::ChatBox;
#[doc(inline)]
pub use chunk_lock::{ChunkHandle, ChunkLock};
#[doc(inline)]
pub use game::Game;
pub use plugin::{Plugin, PluginInfo, Setup};
#[doc(inline)]
pub use world::{World, WorldId};

#[doc(inline)]
pub use libcraft::{
    blocks::{block_data, BlockData, BlockKind, BlockState},
    chunk::{Chunk, ChunkSection},
    inventory::Inventory,
    items::{
        Enchantment, EnchantmentKind, InventorySlot, Item, ItemStack, ItemStackBuilder,
        ItemStackError, ItemStackMeta,
    },
    text::{Text, TextComponentBuilder},
    BlockPosition, ChunkPosition, Position, CHUNK_WIDTH,
};

pub extern crate libcraft;

pub struct PluginLogger {}

impl PluginLogger {
    pub fn info(plugin_name: &'static str, data: String){
        let datetime: OffsetDateTime = match OffsetDateTime::now_local() {
            Ok(x) => x,
            Err(_) => OffsetDateTime::now_utc(),
        };
        println!("\x1b[38;5;31m{} \x1b[35mINFO\x1b[0m  [\x1b[38;5;172mPlugin::\x1b[38;5;86m{}\x1b[0m] {}",
            datetime
            .format(format_description!(
                "[day]/[month]/[year] [hour]:[minute]:[second];[subsecond digits:5]"
            ))
            .unwrap(),
            plugin_name,
            data
        );
    }
    pub fn debug(plugin_name: &'static str, data: String){
        let datetime: OffsetDateTime = match OffsetDateTime::now_local() {
            Ok(x) => x,
            Err(_) => OffsetDateTime::now_utc(),
        };
        println!("\x1b[38;5;31m{} \x1b[32mDEBUG\x1b[0m [\x1b[38;5;172mPlugin::\x1b[38;5;86m{}\x1b[0m] {}",
            datetime
            .format(format_description!(
                "[day]/[month]/[year] [hour]:[minute]:[second];[subsecond digits:5]"
            ))
            .unwrap(),
            plugin_name,
            data
        );
    }
}