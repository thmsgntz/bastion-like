use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SquareMaterials>()
            .add_startup_system(create_board);
    }
}

pub struct SquareMaterials {
    //highlight_color: Handle<StandardMaterial>,
    //selected_color: Handle<StandardMaterial>,
    black_color: Handle<StandardMaterial>,
    white_color: Handle<StandardMaterial>,
}

impl FromWorld for SquareMaterials {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        SquareMaterials {
            //highlight_color: materials.add(Color::rgb(0.8, 0.3, 0.3).into()),
            //selected_color: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
            black_color: materials.add(Color::rgb(0., 0.1, 0.1).into()),
            white_color: materials.add(Color::rgb(1., 0.9, 0.9).into()),
        }
    }
}

fn setup_castle(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -5.0, 0.0),
                scale: Vec3::ONE * 0.5,
                ..default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/Castle/Castle_FBX.gltf#Scene0"));
        });
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mat: ResMut<Assets<StandardMaterial>>,
    materials: Res<SquareMaterials>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));

    for i in 0..8 {
        for j in 0..8 {
            let initial_mat = if (i + j + 1) % 2 == 0 {
                materials.white_color.clone()
            } else {
                materials.black_color.clone()
            };

            commands.spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: initial_mat.clone(),
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..Default::default()
            });
        }
    }

    let mesh_plane = meshes.add(Mesh::from(shape::Plane { size: 8. }));
    let mat = mat.add(StandardMaterial { ..default() });

    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(4.0, 0.1, 4.0))
        .insert_bundle(PbrBundle {
            mesh: mesh_plane.clone(),
            material: mat.clone(),
            transform: Transform::from_xyz(3.5, 0.0, 3.5),
            global_transform: Default::default(),
            visibility: Visibility { is_visible: false },
            computed_visibility: Default::default(),
        });
}
