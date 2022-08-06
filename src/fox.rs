use bevy::prelude::*;

pub struct FoxPlugin;
impl Plugin for FoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(link_animations)
            .add_system(keyboard_control)
        ;
    }
}
pub struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Component)]
pub struct Creature;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        asset_server.load("models/Fox.glb#Animation2"),
        asset_server.load("models/Fox.glb#Animation1"),
        asset_server.load("models/Fox.glb#Animation0"),
    ]));

    commands.insert_resource(Animations(vec![
        asset_server.load("models/skeleton/scene.gltf#Animation1"),
        asset_server.load("models/skeleton/scene.gltf#Animation2"),
        asset_server.load("models/skeleton/scene.gltf#Animation3"),
    ]));

    // Fox
    let id_fox = commands
        .spawn()
        .insert_bundle(PbrBundle {
            transform: Transform {
                translation: Vec3::new(4.0, 0.0, 4.0),
                scale: Vec3::ONE * 0.01,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/Fox.glb#Scene0"));
    })
        .insert(Creature)
        .id()
        ;

    // Skeleton
    let id_sk = commands
        .spawn().insert_bundle(PbrBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::ONE * 0.6,
            ..default()
        },
        ..default()
    })
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/skeleton/scene.gltf#Scene0"));
    })
        .insert(Creature)
        .id()
        ;

    info!("Fox id: {}", id_fox.id());
    info!("Ske id: {}", id_sk.id());

}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut animation_players: Query<(Entity, &mut AnimationPlayer)>,
    mut done: Local<u8>,
) {
    if *done < 2 {
        for (entity, mut player) in animation_players.iter_mut() {
            player.play(animations.0[0].clone_weak()).repeat();
            *done += 1;
        }
    }
}

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

pub fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the heirachy

    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);

        info!("Calling: link_animations. {:#?}", entity);

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animationsplayers for the same top parent");
        } else {
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity.clone()));
        }
    }
}

pub fn sync_animation_to_velocity(
    mut player_query: Query<&mut AnimationPlayer>,
    //mut query: Query<(&Movable, &mut AnimationConfig, &AnimationEntityLink), Changed<Movable>>,
    mut query: Query<(/*&Movable, &mut AnimationConfig,*/ &AnimationEntityLink), Changed<Creature>>,
    animations: Res<Animations>,
    //game_speed: Res<GameSpeed>,
) {
    /*for (movable, mut animation_config, animation_entity) in query.iter_mut() {
        if let Ok(mut player) = player_query.get_mut(animation_entity.0) {
//Stuff
        }
    }*/
    let i = 1;
}

/*
fn keyboard_animation_control(
    keyboard_input_res: Res<Input<KeyCode>>,
    animations: Res<Animations>,
    mut query_animation: Query<&mut AnimationPlayer>,
    mut query_skelly: Query<(Entity, &mut Transform), With<Creature>>

) {
    if let Ok(mut player) = query_animation.get_single_mut() {
        let keyboard_input = &mut keyboard_input_res.into_inner();
        let (mut skelly_transform, mut skelly_velocity, mut skelly) = query_skelly.get_single_mut().unwrap();

        let pressed_keys = direction::get_pressed_keys_of_interest(keyboard_input);
        let mut vector_direction = Vec3::ZERO;
        let mut vector_angular = Vec3::ZERO;
        let mut is_action = SkellyAnimationId::None;
        let mut is_shift = 0.0;

        if !skelly.is_ready(&mut player) {
            return;
        }

        for key in pressed_keys {
            info!("Keys pressed:{:#?}", key);
            match key {
                KeyCode::Up => {
                    vector_direction += Vec3::new(1.0, 0.0, 1.0);
                },
                KeyCode::Right => {
                    vector_direction += Vec3::new(-1.0, 0.0, 1.0);
                    vector_angular = Vec3::new(0.0, -1.0, 0.0);
                },
                KeyCode::Down => {
                    vector_direction += Vec3::new(-1.0, 0.0, -1.0);
                },
                KeyCode::Left => {
                    vector_direction += Vec3::new(1.0, 0.0, -1.0);
                    vector_angular = Vec3::new(0.0, 1.0, 0.0);
                },
                KeyCode::LShift => {
                    is_shift = 1.0;
                },
                KeyCode::Numpad1 => {
                    is_action = SkellyAnimationId::Yell;
                },
                KeyCode::Numpad2 => {
                    is_action = SkellyAnimationId::Attack;
                },
                KeyCode::Numpad3 => {
                    is_action = SkellyAnimationId::Fall;
                },
                KeyCode::Numpad4 => {
                    is_action = SkellyAnimationId::Hit;
                },
                KeyCode::Numpad5 => {
                    is_action = SkellyAnimationId::Die;
                },
                KeyCode::Numpad6 => {
                    is_action = SkellyAnimationId::Spawn;
                },
                KeyCode::Numpad7 => {
                    // is_action = SkellyAnimationId::Hanged;
                    info!("skelly_velocity:{:#?}", skelly_velocity);
                },
                _ => {}
            }
        }
    }
}*/

fn keyboard_control(
    keyboard_input_res: Res<Input<KeyCode>>,
    animations: Res<Animations>,
    query: Query<(Entity, &AnimationEntityLink), With<Creature>>,
    mut done: Local<u8>
) {
    if *done != 2 {
        *done = 0;
        for (entity, animation_entity) in query.iter() {
            info!("keyboard: entity: {:#?}, animationEntity.0: {:#?}", entity, animation_entity.0);
            info!("existing");
            *done +=1 ;
        }
    }
}