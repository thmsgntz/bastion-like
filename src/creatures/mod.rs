use bevy::prelude::*;
use bevy::utils::HashMap;

mod skelly;

pub const ENTITY_SPEED: f32 = 2.0;
pub const ENTITY_SPEED_ROTATION: f32 = 0.1;

pub struct CreaturePlugin;
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(skelly::SkellyPlugin);
    }
}

//#[derive(Bundle, Clone)]

//#[derive(Bundle)]
#[derive(Component)]
pub struct Creature(pub String);
//#[bundle]
//pub transform: PbrBundle,
// ajouter Transform
// ajouter scene

