//! A starting point for your own mod. Copy this folder, rename it, and replace the
//! examples with your stuff.
//!
//! Rename checklist:
//! - `Cargo.toml`: `name` is your mod id (and the library's file name and the mod folder's),
//!   `version` the version players see.
//! - `declare_mod!`: `deps`, the mods yours needs.
//! - `assets/lang/*.lang`: the keys start with your mod id; `mod.<id>.name` is the name players
//!   see (without it, the id).
//!
//! Build: `cargo build --release`. Then make `run/mods/<your id>/` with the library from
//! `target/release/` and the `assets` folder next to it.
//! To share it, run `weft --pack-mod run/mods/<your id>`,
//! it packs that folder (library included) into a zip.
//!
//! This example adds one item, a setting, and a gift for joined players.
//! Everything else is commented examples. The full cheat sheet is at `docs/modding.md`.
//! `mods/base` and `mods/testmod` use exactly this API, so for more examples check those.
//! 
//! I'd like to implement a version of Threat for each hostile mob, akin to World of Warcraft. Every mob should have an amount of Threat for every player currently nearby. They aggro whichever player's Threat is the highest.
//! Events like damage done by the player, or healing done by the player, should be able to change the amount of Threat the enemy has towards that player, and if that makes one player suddenly the new number one, the mob should change aggro target.
//! How should I implement this as a mod in Weft in the most extensible way possible? It should be easy to adjust the values for what threatens a mob type and how much, easy for new mobs to hook into and use the system, and when initially implemented look basically the same as works now, which is similar to Minecraft.
//! Some initial ideas on what adds Threat: seeing the player, player damage, player healing.
//!

use weft_sdk::*;

declare_mod!(deps: ["base"], init: init);

/// Called once at startup, in parallel with the other mods: register everything here.
/// Nothing exists in the world yet, you only describe what your mod adds.
fn init(r: &mut Registry) {
    // config(r);
    // blocks(r);
    // items(r);
    // recipes(r);
    // worldgen(r);
    entities(r);
    events(r);
    // commands(r);
    client(r);
}

/// Settings show up in Settings > Mod Settings by themselves and are saved to
/// `config/examplemod.txt`. Name and tooltip come from `config.examplemod.<key>` in the lang file.
// fn config(r: &mut Registry) {
//
// }

/// Mobs and other entities
/// Models in code (`humanoid`, `quadruped`, `spider`) or Blockbench files in `assets/models`
/// More at `mods/base/src/mobs.rs` (AI goals, spawn rules, etc.).
fn entities(_r: &mut Registry) {

}

/// Reacting to the game, events arrive batched once per tick
/// for "may this happen" use `r.before_event`
/// to change a number use `r.modify`
/// For more info check "Hooking into the game" in `docs/modding.md`
fn events(r: &mut Registry) {

}

/// Client side: screens, HUD, key bindings, packets.
/// More at:
/// `mods/base/src/player.rs` (HUD),
/// `mods/testmod/src/lib.rs` (screens, block entity panels),
/// `mods/testmod/src/looks.rs` (particles, shaders, sounds).
fn client(_r: &mut Registry) {

}
