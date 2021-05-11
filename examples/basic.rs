use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_hud_pass::*;

fn main() {
    AppBuilder::default()
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system.system())
        .add_plugin(HUDPassPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // NOTE: The HUDPbrBundle is just a PbrBundle with a HUDPass component instead
    // of a MainPass component, for use in the HUD pass and NOT the main pass
    commands.spawn_bundle(HUDPbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::RED,
            ..Default::default()
        }),
        transform: Transform::from_translation(Vec3::ZERO),
        ..Default::default()
    });

    // NOTE: The HUDCameraBundle is just a PerspectiveCameraBundle with the correct name
    // for use in the HUD pass
    commands.spawn_bundle(HUDCameraBundle {
        transform: Transform::from_translation(Vec3::new(2.0, 2.0, 4.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(5.0, 5.0, 6.0),
        ..Default::default()
    });
}
