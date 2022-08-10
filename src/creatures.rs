use bevy::prelude::*;
use bevy::utils::HashMap;

pub const ENTITY_SPEED: f32 = 2.0;
pub const ENTITY_SPEED_ROTATION: f32 = 0.1;

//#[derive(Bundle, Clone)]

#[derive(Bundle)]
pub struct Creature  {
    pub hashmap_animations: HashMapAnimations,
    //pub direction_vec3: Vec3, // TODO: à bouger, c'est très moche ici

    #[bundle]
    pub transform: PbrBundle,
    // ajouter Transform
    // ajouter scene
}

#[derive(Component)]
pub struct HashMapAnimations {
    pub current_animation_id: i32,
    pub hash_animations: HashMap<i32, SingleAnimation>,
}

impl Default for HashMapAnimations {
    fn default() -> Self {
        HashMapAnimations {
            current_animation_id: 1,
            hash_animations: Default::default(),
        }
    }
}

impl HashMapAnimations {
    pub fn add_animation(
        &mut self,
        id: i32,
        handle: Handle<AnimationClip>,
        duration: f32,
        is_repeatable: bool,
    ) {
        self.hash_animations
            .insert(
                id,
                SingleAnimation {
                    id,
                    handle,
                    duration,
                    is_repeatable
                }
            );
    }

    pub fn get(
        &self,
        id: i32
    ) -> Option<&SingleAnimation> {
        self.hash_animations.get(&id)
    }
}

pub struct SingleAnimation {
    pub id: i32,
    pub handle: Handle<AnimationClip>,
    pub duration: f32,
    pub is_repeatable: bool,
}

// impl Animation<T> {
//     pub fn new(
//         id: T,
//         handle: Handle<AnimationClip>,
//         duration: f32,
//         is_repeatable: bool,
//     ) -> Animation<T> {
//         Animation {
//             id,
//             handle,
//             duration,
//             is_repeatable,
//         }
//     }
// }
