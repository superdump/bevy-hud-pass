use bevy::{
    input::{keyboard::KeyCode, system::exit_on_esc_system, Input},
    prelude::*,
    render::render_graph::base::MainPass,
};
use bevy_hud_pass::*;

fn main() {
    AppBuilder::default()
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system.system())
        .add_plugin(HUDPassPlugin)
        .add_startup_system(setup.system())
        .add_system(toggle_passes.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(HUDPbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::RED,
            ..Default::default()
        }),
        transform: Transform::from_translation(Vec3::ZERO),
        ..Default::default()
    });

    // NOTE: The main pass camera is to the left, which means the cube is visible on the right
    // when rendered by the main pass
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(-2.0, 2.0, 4.0))
            .looking_at(Vec3::new(-2.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });
    // NOTE: The HUD pass camera is to the right, which means the cube is visible on the left
    // when rendered by the main pass
    commands.spawn_bundle(HUDCameraBundle {
        transform: Transform::from_translation(Vec3::new(2.0, 2.0, 4.0))
            .looking_at(Vec3::new(2.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(5.0, 5.0, 6.0),
        ..Default::default()
    });

    // Set up UI labels for clarity
    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(32.0),
                top: Val::Px(32.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            "HUD pass\n('h' to toggle)",
            TextStyle {
                font: font_handle.clone(),
                font_size: 100.0,
                color: Color::WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    });
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                right: Val::Px(32.0),
                top: Val::Px(32.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            "Main pass\n('m' to toggle)",
            TextStyle {
                font: font_handle.clone(),
                font_size: 100.0,
                color: Color::WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    });
    commands.spawn_bundle(UiCameraBundle::default());
}

fn toggle_passes(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<(Entity, Option<&HUDPass>, Option<&MainPass>), With<Handle<Mesh>>>,
) {
    for (cube_entity, hud_pass, main_pass) in query.iter() {
        if keyboard_input.just_pressed(KeyCode::H) {
            if hud_pass.is_some() {
                info!(" - HUD pass");
                commands.entity(cube_entity).remove::<HUDPass>();
            } else {
                info!(" + HUD pass");
                // NOTE: Add the HUDPass component to render this mesh in this plugin's HUD pass
                commands.entity(cube_entity).insert(HUDPass);
            }
        }

        if keyboard_input.just_released(KeyCode::M) {
            if main_pass.is_some() {
                info!(" - Main pass");
                // NOTE: Remove the MainPass component to NOT render this mesh in the bevy main pass
                commands.entity(cube_entity).remove::<MainPass>();
            } else {
                info!(" + Main pass");
                commands.entity(cube_entity).insert(MainPass);
            }
        }
    }
}
