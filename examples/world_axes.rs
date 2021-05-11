use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_hud_pass::{
    world_axes::{WorldAxesPlugin, WorldAxesPositionTag, WorldAxesRotationTag},
    HUDCameraBundle, HUDPassPlugin,
};

fn main() {
    AppBuilder::default()
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system.system())
        .add_plugin(HUDPassPlugin)
        .add_plugin(WorldAxesPlugin)
        .add_plugin(FlyCameraPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // NOTE: The HUDCameraBundle is just a PerspectiveCameraBundle with the correct name
    // for use in the HUD pass
    commands
        .spawn_bundle(HUDCameraBundle::default())
        // NOTE: Tag the HUD camera with WorldAxesPositionTag to draw the world axes in this camera view
        .insert(WorldAxesPositionTag);

    // Add a regular perspective camera and the fly camera component so we can fly around
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 5.0, 5.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert_bundle((FlyCamera::default(), WorldAxesRotationTag));

    // Add some regular PbrBundle cubes for something to look at in the world
    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let red = materials.add(StandardMaterial {
        base_color: Color::RED,
        ..Default::default()
    });
    for z in -1..=1 {
        for x in -1..=1 {
            commands.spawn_bundle(PbrBundle {
                mesh: cube_handle.clone(),
                material: red.clone(),
                transform: Transform::from_xyz(x as f32 * 10.0, 0.5, z as f32 * 10.0),
                ..Default::default()
            });
        }
    }
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::ANTIQUE_WHITE,
            ..Default::default()
        }),
        // transform: Transform::from_xyz(x as f32 * 10.0, 0.0, z as f32 * 10.0),
        ..Default::default()
    });

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(-5.0, 5.0, 6.0),
        ..Default::default()
    });
}
