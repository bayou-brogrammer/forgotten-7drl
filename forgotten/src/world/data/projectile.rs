use crate::prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CollidesWith {
    pub solid: bool,
    pub character: bool,
}

impl Default for CollidesWith {
    fn default() -> Self {
        Self { solid: true, character: false }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OnCollision {
    Remove,
    // Explode(explosion_spec::Explosion),
    RemoveRealtime,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProjectileDamage {
    pub pen: u32,
    pub hit_points: u32,
    pub push_back: bool,
    pub life_steal: bool,
    pub hull_pen_percent: u32,
    pub weapon_name: Option<WeaponType>,
}
