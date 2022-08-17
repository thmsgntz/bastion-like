
use bevy::prelude::*;
use crate::animations_handler::{AddAnimation, ChangeAnimation};
use crate::creatures::skelly::Skelly;

pub(crate) mod skelly;
mod chess_pieces;
pub(crate) mod mob;

/*
TODO:
 0.5: Gérer l'enchainement des animations.
    -> hashmap (duration, animationClip) ok
    -> stopwatch ok
 1. Déplacer les fonctions de skeleton.rs sur les déplacements de Skelly ici
 2. Essayer d'en faire des functions génériques?
 */

const ENTITY_SPEED: f32 = 2.0;
const ENTITY_SPEED_ROTATION: f32 = 0.1;

pub trait CreatureTrait {
    fn spawn(commands: Commands, asset_server: Res<AssetServer>, event_writer: EventWriter<AddAnimation>);

    fn update_animation(target: u32, index_animation: usize, event_writer: &mut EventWriter<ChangeAnimation>);
}

/// Player marker
#[derive(Component)]
pub(crate) struct Player;

pub struct CreaturePlugin;
impl Plugin for CreaturePlugin {
    fn build(&self, _app: &mut App) {}
}

//#[derive(Bundle, Clone)]

#[derive(Component, Copy, Clone)]
/// Contient l'index de l'animation en cours
/// Mis à jour par animations_handler:update_animation
pub struct CurrentAnimationIndex(pub usize);

impl From<usize> for CurrentAnimationIndex {
    fn from(a: usize) -> Self {
        Self(a)
    }
}


pub enum TypeCreature {
    Skelly,
    Gollum
}

//#[derive(Bundle)]
#[derive(Component)]
pub struct Creature{
    pub type_creature: TypeCreature,

    /// index (in vec_animations)  of current animation being played
    pub current_animation_index: CurrentAnimationIndex,
}

impl Creature {
    pub fn update_animation(&self, target: u32, index_animation: usize, event_writer: &mut EventWriter<ChangeAnimation>)
    {
        match self.type_creature {
            TypeCreature::Skelly => {
                info!("Calling update_animation skelly");
                Skelly::update_animation(target, index_animation, event_writer);
            }
            TypeCreature::Gollum => {}
        }
    }

}
//#[bundle]
//pub transform: PbrBundle,
// ajouter Transform
// ajouter scene

