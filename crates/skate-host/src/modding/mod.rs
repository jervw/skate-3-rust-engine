//! The sim-facing part of upstream's `modding` module (S1SKATE.md). The
//! skater's physics reads mod-owned joint and part overrides every tick;
//! headless no mod runs, so they stay empty and the physics is stock. The
//! Lua mod runtime (`skate-mods`) and the rest of `modding` are cut.
pub(crate) mod extensions;
pub(crate) mod player_physics;
