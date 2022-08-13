use crate::creatures::Creature;
use bevy::prelude::*;

pub struct AnimationHandler;
impl Plugin for AnimationHandler {
    fn build(&self, app: &mut App) {
        app.insert_resource::<VecSceneHandle>(Default::default())
            .add_event::<ChangeAnimation>()
            .add_event::<AddAnimation>()
            .add_event::<RemoveAnimation>()
            .add_system_to_stage(CoreStage::PostUpdate, add_animation)
            .add_system_to_stage(CoreStage::PostUpdate, change_animation)
            .add_system_to_stage(CoreStage::PostUpdate, remove_animation);
    }
}

/// Event utilisé par change_animation() pour changer d'animation
/// # Examples
/// ```
/// event_writer.send(
///    ChangeAnimation{
///        target: entity,
///        index: number as usize,
///        repeat: true,
///    }
/// );
/// ```
#[derive(Debug)]
pub struct ChangeAnimation {
    pub(crate) target: Entity,
    pub(crate) index: usize,
    pub(crate) repeat: bool
}

pub struct AddAnimation {
    pub scene_handler: SceneHandle
}

pub struct RemoveAnimation {
    pub entity_id: Entity,
}

/// Ressource qui contient un vecteur de SceneHandle
/// qui définit tous les animations des créatures
pub struct VecSceneHandle(pub Vec<SceneHandle>);

impl Default for VecSceneHandle {
    fn default() -> Self {
        Self {
            0: vec![]
        }
    }
}

#[derive(Clone, Debug)]
pub struct SceneHandle {
    pub handle: Handle<Scene>,
    pub vec_animations: Vec<Handle<AnimationClip>>,
    //is_loaded: bool,
    pub creature_entity_id: Option<u32>, // creature id associated with this scene
}

#[derive(Component)]
pub struct AnimationDuration {
    pub time: Timer,
}

/// Composant qui mis à jour par link_animations()
/// L'entité est l'id de l'AnimationPlayer.
/// en utilisant ces deux queries :
/// ```
///    mut query_player: Query<&mut AnimationPlayer>,
///    mut query_entity: Query<(Entity, &AnimationEntityLink), With<Creature>>,
/// ```
/// On peut retrouver, pour une entité, son AnimationEntityLink, donc l'id de son AnimationPlayer
/// et:
/// ```
///    Ok(player) = query_player.get_mut(animation_link.0)
/// ```
#[derive(Component)]
pub struct AnimationEntityLink(pub Entity);

fn get_top_parent(mut curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    //Loop up all the way to the top parent
    loop {
        if let Ok(parent) = parent_query.get(curr_entity) {
            curr_entity = parent.0;
        } else {
            break;
        }
    }
    curr_entity
}

/// Fonction qui lie une entité avec son AnimationPlayer par le composant AnimationEntityLink.
/// Voir: https://github.com/bevyengine/bevy/discussions/5564#discussion-4275825
pub fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the hierarchy

    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);

        debug!("Calling: link_animations. {:#?}", entity);

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animations players for the same top parent");
        } else {
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity.clone()));
        }
    }
}

/// Fonction qui lit un Event ChangeAnimation et :
///   1. D'après l'id de l'entité à animer (event.target.id())
///   2. Retrouver l'animationPlayer associé en parcourant les tuples (Entity, &AnimationEntityLink)
///      ```
///         creature.id() == event_creature_à_animer.target.id()
///      ```
///   3. Une fois le player retrouvé, on cherche les animations dans VecSceneHandle
///      ```
///         scene_handler_random_creature.id() == event_creature_à_animer.id()
///      ```
fn change_animation(
    mut events: EventReader<ChangeAnimation>,
    scene_handlers: Res<VecSceneHandle>,
    mut query_player: Query<&mut AnimationPlayer>,
    mut query_entity: Query<(Entity, &AnimationEntityLink), With<Creature>>,
) {
    for event in events.iter() {
        // retrouver l'entity
        debug!("change_animation::Event found! {:#?}", event);
        for (entity, animation_link) in query_entity.iter_mut() {
            if entity.id() == event.target.id() {  // on a retrouvé le player associé à l'entité
                for scene_handler in &scene_handlers.0 {
                    if scene_handler.creature_entity_id == Some(entity.id()) {  // on retrouve ses animations SceneHandler
                        if let Ok(mut player) = query_player.get_mut(animation_link.0) {
                            if event.repeat {
                                player.play(scene_handler.vec_animations[event.index].clone_weak());
                            } else {
                                player.play(scene_handler.vec_animations[event.index].clone_weak()).repeat();
                            }
                        }
                    }
                }
            }
        }
    }
}


fn add_animation(
    mut events: EventReader<AddAnimation>,
    mut vec_scene_handlers: ResMut<VecSceneHandle>,
) {
    for event in events.iter() {
        debug!("AddAnimation: {:#?}", event.scene_handler);
        vec_scene_handlers.0.push(
            event.scene_handler.clone()
        )
    }
}

fn remove_animation(
    mut events: EventReader<RemoveAnimation>,
    mut vec_scene_handlers: ResMut<VecSceneHandle>,
) {
    let mut found = false;
    for event in events.iter() {
        found = false;
        for i in 0..vec_scene_handlers.0.len() {
            if !found && vec_scene_handlers.0[i].creature_entity_id == Some(event.entity_id.id()) {
                debug!("Remove_animation: entity found, removing.");
                found = true;
                vec_scene_handlers.0.swap_remove(i);
            }
        }
        if !found {
            warn!("Remove_animation: called with unknown entity")
        }
    }
}