use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::shape::Icosphere;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_physics);
            //.add_system(print_ball_altitude);
    }
}


fn setup_physics(
    mut commands: Commands,
    mut asset_meshes: ResMut<Assets<Mesh>>,
    mut asset_materials: ResMut<Assets<StandardMaterial>>,
) {

    let ball_mesh = asset_meshes.add(Mesh::from(Icosphere{ radius: 0.5, subdivisions: 2 }));
    let ball_material = asset_materials.add(StandardMaterial::from(Color::rgb(0., 0.1, 0.1)));

    /* Create the bouncing ball. */
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(GravityScale(1.0))
        .insert(ColliderMassProperties::Density(1.0))
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert_bundle(
            PbrBundle {
                mesh: ball_mesh.clone(),
                material: ball_material.clone(),
                transform: Transform::from_xyz(4.0, 10.0, 4.0),
                global_transform: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default()
            }
        )
    ;
}

fn print_ball_altitude(positions: Query<(&Transform, &Velocity), With<RigidBody>>) {
    for (transform, velocity) in positions.iter() {
        println!("Ball altitude: {} // velocity {:#?}", transform.translation.y, velocity);
    }
}