//! The two override types of upstream's `skate_mods::extensions` that the
//! skater's physics stores, copied unchanged from `crates/skate-mods` at
//! 4488651 so that skate-host does not link the Lua runtime (S1SKATE.md).
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JointOverride {
    #[serde(default)] pub swing_limit: Option<f32>,
    #[serde(default)] pub twist_limit: Option<f32>,
    #[serde(default)] pub free_swing: Option<bool>,
    #[serde(default)] pub free_twist: Option<bool>,
    #[serde(default)] pub drive_enabled: Option<bool>,
    #[serde(default)] pub enabled: Option<bool>,
    #[serde(default)] pub descendants: bool,
    #[serde(default)] pub possession_enabled: Option<bool>,
}
impl JointOverride {
    pub fn validate(&self) -> bool {
        [self.swing_limit,self.twist_limit].iter().all(|v| v.is_none_or(|v| v.is_finite() && (0.01..=std::f32::consts::PI).contains(&v)))
    }
}

#[derive(Clone,Debug,Default,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartOverride {
    pub motion:Option<String>, pub collision:Option<bool>, pub friction:Option<f32>,
    pub animation_drives:Option<bool>, pub possession_drives:Option<bool>,
}
impl PartOverride {
    pub fn validate(&self)->bool {self.motion.as_deref().is_none_or(|m|matches!(m,"dynamic"|"frozen"|"static")) && self.friction.is_none_or(|f|f.is_finite() && (0.0..=10.0).contains(&f))}
}
