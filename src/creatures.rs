use bevy::prelude::*;
use bevy::utils::HashMap;

pub const ENTITY_SPEED: f32 = 2.0;
pub const ENTITY_SPEED_ROTATION: f32 = 0.1;

//#[derive(Bundle, Clone)]

#[derive(Bundle)]
pub struct Creature  {
    #[bundle]
    pub transform: PbrBundle,
    // ajouter Transform
    // ajouter scene
}
