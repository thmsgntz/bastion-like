use bevy::prelude::*;
use bevy::utils::HashMap;

mod skelly;
mod chess_pieces;

/*
TODO:
 0,5. Faire spawn des mobs un après l'autre, pour voir si animations_handler marche bien
 1. Déplacer les fonctions de skeleton.rs sur les déplacements de Skelly ici
 2. Essayer d'en faire des functions génériques?

 */

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

