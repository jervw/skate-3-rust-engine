# Branch `s1skate-headless`: a headless cut of SK8-ENGINE for Schedule I × Skate 3

This branch is SK8-ENGINE/skate-3-rust-engine at **`4488651`** (2026-10-03,
the first revision with both the GPL-3.0 `LICENSE` and the README grant)
with rendering, the app, audio, menus, multiplayer and the Lua mod runtime
removed, and a host API added. It is consumed by
schedule-skate, a Schedule I mod, as a Cargo
git dependency pinned by `rev`: packages `skate-host`, `skate-core` and
`skate-data`. Nothing else is changed. Physics is not tuned here.

The cut is the one chasmlol made of `cb79689` in
[chasmlol/2010-rust-rewrite-mashup](https://github.com/chasmlol/2010-rust-rewrite-mashup)
(`skate/crates` at `f608f85`), done again on this revision.

## Licenses

- Upstream's code is **GPL-3.0-only** (`LICENSE`, `README.md`), as it is on
  upstream's `main`. `LICENSE` is upstream's, unchanged.
- The mashup's changes and additions (listed below) are chasmlol's work,
  published under **Apache-2.0**. Its license and NOTICE are in
  `licenses/2010-rust-rewrite-mashup/`, unchanged. Apache-2.0 code can be
  combined into a GPLv3 work; the combination is distributed under GPLv3.
  This is compatibility reasoning, not legal advice.
- This branch's own changes (marked "this branch" below) are offered under
  GPL-3.0-or-later.

## The cut, as four commits

| Commit | What |
|---|---|
| 1/4 | Remove what the mashup removed from `crates/skate-game` (70 files and folders: app, rendering, shaders, audio, HUD, menus, multiplayer, replay, modding, updater, crash UI, `src/tests`), and the crates only they use (`skate-mods`, `skate-steam-relay`) |
| 2/4 | Rename `crates/skate-game` to `crates/skate-host` (a pure rename) |
| 3/4 | The mashup's changes to upstream files, merged with upstream's changes since `cb79689` |
| 4/4 | The bridge, `lib.rs`, skate-host's manifest, the workspace |

**Proof of the procedure.** The same four commits applied to `cb79689` (a
local branch, not published) give the mashup's `skate/crates` file for file
(apart from upstream's own tests and examples, which the mashup dropped), and
the Schedule I mod's 28 headless rides (7 inputs on 4 worlds, every bone of
every tick) were bit-identical against the mashup's `f608f85`.

## Files carrying the mashup's Apache-2.0 work

From `f608f85`, then merged with upstream's changes up to `4488651`:

- `crates/skate-core/src/physics/board_world.rs` and its test
  `physics/tests/board_world.rs`: imported floor seams.
- `crates/skate-host/Cargo.toml` (this branch adds `skate-dynamics`).
- `crates/skate-host/src/`: `animation.rs`, `camera.rs`, `crash_context.rs`,
  `grind_world.rs`, `input.rs`, `input/controllers.rs`, `physics.rs`,
  `physics/network.rs`, `physics/offboard/contact_toolkit.rs`,
  `physics/solve.rs`, `physics/solve/assembly_contacts.rs`,
  `physics/solve/diagnostics.rs`, `scoring_runtime.rs`,
  `session_marker/mod.rs`, `skate_world.rs`, `skater_animation.rs`.
- New: `crates/skate-host/src/physics/bridge.rs` (`bridge::Session`, the host
  API), `src/lib.rs`, `src/bridge.rs`.

## How conflicts with upstream were resolved

Rule: where upstream changed code the mashup kept, upstream's logic wins;
where the mashup deleted app or rendering code that upstream later edited,
the deletion stands; `bridge::Session`'s public interface stays as at
`f608f85`.

| File | Resolution |
|---|---|
| `physics.rs`, `physics/frame.rs`, `physics/ground_runtime/settings.rs` | Upstream removed the trainer tuning (`skate_mods::TrainerTuning`) and vehicle ejection; the mashup had only renamed the type. Upstream's version; the mashup's `tuning.rs` is not carried. The app's `PhysicsPlugin`, `advance` and `present` systems are removed, as by the mashup |
| `physics/network.rs` | Upstream replaced vehicle proxies with mod solids (`append_solid`, `skate-dynamics`). Upstream's version: the solids are woven into kept physics (`biped_air`, `biped_ground`, `solve`, `skeleton_feedback`, `air_selector`, `solid_contacts`) and are empty headless, so `skate-dynamics` stays linked |
| `scoring_runtime.rs` | Upstream's new accessors (trick name, scores, stance) kept; `hud_input` removed, as by the mashup |
| `session_marker/mod.rs` | The mashup's `Runtime`; upstream removed the marker sounds, and the mashup's log lines that replaced them go with them |
| `skate_world.rs` | The mashup's collision-only file, plus upstream's two collision changes: static clusters register with matching id −1 (upstream PR #17) and two log lines are gone. Upstream's new renderer is not carried |
| `animation.rs`, `camera.rs`, `input.rs` | The mashup's deletions (vehicle poses, the camera plugin, the menu-gated `publish_actions` system) |

## This branch's own changes

- `physics/bridge.rs`: `Session::new` loads easy through
  `GamePhysics::load_with_difficulty(.., Difficulty::Easy)`, because upstream
  removed `load_with_map`, which did exactly that. `src/bridge.rs` names
  `4488651`.
- `lib.rs`: adds `custom_difficulty` (upstream's, unchanged; the physics
  loads its collections through it, and without
  `<data>/../settings/custom-difficulty.json` it changes nothing) and
  `modding`; drops `tuning`.
- `modding/`: upstream's physics reads mod-owned joint and part overrides
  every tick. `modding/player_physics.rs` is upstream's file with the Lua
  API's functions (`set`, `clear`, `snapshot`, `impulse`, `set_part`) and
  their tests removed; the rest is unchanged. `modding/extensions.rs`
  carries `JointOverride` and `PartOverride` unchanged from
  `crates/skate-mods/src/extensions.rs`, so that skate-host does not link
  the Lua runtime. With no mod, the overrides stay empty and the physics is
  stock. `physics/skater.rs` names these two types by their new path.
- Root `Cargo.toml`: members `skate-core`, `skate-data`, `skate-dynamics`,
  `skate-net`; `skate-host` is its own workspace, as in the mashup.
  `Cargo.lock` follows.

## Checks on this branch

- `skate-host` builds headless on Linux and cross-compiles for
  `x86_64-pc-windows-gnu` (inside schedule-skate's `skate_core.dll`).
- `cargo test -p skate-core`: the same result as unmodified `4488651`: all
  pass except two that also fail there
  (`board_world::broadphase_tests::predictive_contacts_and_retention_match_full_scan_for_every_primitive`,
  `skeleton_body::collision_feedback_tests::a_moving_group_8_body_reaches_native_impact_feedback_for_a_stationary_actor`).
