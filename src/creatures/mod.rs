use bevy::prelude::*;
use crate::animations_handler::AddAnimation;

pub(crate) mod skelly;
mod chess_pieces;
pub(crate) mod mob;

/*
TODO:
 1. Déplacer les fonctions de skeleton.rs sur les déplacements de Skelly ici
 2. Essayer d'en faire des functions génériques?
 */

const ENTITY_SPEED: f32 = 2.0;
const ENTITY_SPEED_ROTATION: f32 = 0.1;

pub trait CreatureTrait {
    fn spawn(commands: Commands, asset_server: Res<AssetServer>, event_writer: EventWriter<AddAnimation>);
}

/// Player marker
#[derive(Component)]
pub(crate) struct Player;

pub struct CreaturePlugin;
impl Plugin for CreaturePlugin {
    fn build(&self, _app: &mut App) {}
}

//#[derive(Bundle, Clone)]

//#[derive(Bundle)]
#[derive(Component)]
pub struct Creature(pub String);
//#[bundle]
//pub transform: PbrBundle,
// ajouter Transform
// ajouter scene

