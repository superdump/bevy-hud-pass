use crate::HUDPbrBundle;
use bevy::{prelude::*, render::camera::Camera, transform::TransformSystem};

pub struct WorldAxesPlugin;

impl Plugin for WorldAxesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<WorldAxes>()
            .add_startup_system(world_axes_setup_system.system())
            .add_system(world_axes_toggle_system.system())
            .add_system_to_stage(
                CoreStage::PostUpdate,
                world_axes_system
                    .system()
                    .after(TransformSystem::TransformPropagate),
            );
    }
}

pub struct WorldAxes {
    pub enabled: bool,
    pub clip_space_position: Vec3,
    pub scale: f32,
    pub axes_entity: Option<Entity>,
    pub axis_mesh: Option<Handle<Mesh>>,
    pub standard_materials: Vec<Handle<StandardMaterial>>,
}

impl Default for WorldAxes {
    fn default() -> Self {
        WorldAxes {
            enabled: true,
            clip_space_position: Vec3::new(0.73044837, -0.59729564, 0.2318211),
            scale: 0.1f32,
            axes_entity: None,
            axis_mesh: None,
            standard_materials: Vec::with_capacity(3),
        }
    }
}

pub struct WorldAxesTag;
pub struct WorldAxesPositionTag;
pub struct WorldAxesRotationTag;

fn world_axes_setup_system(
    mut commands: Commands,
    mut world_axes: ResMut<WorldAxes>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    world_axes.axis_mesh = Some(meshes.add(Mesh::from(shape::Capsule {
        radius: 0.03f32,
        ..Default::default()
    })));

    let red = standard_materials.add(Color::RED.into());
    world_axes.standard_materials.push(red);
    let green = standard_materials.add(Color::GREEN.into());
    world_axes.standard_materials.push(green);
    let blue = standard_materials.add(Color::BLUE.into());
    world_axes.standard_materials.push(blue);

    if world_axes.enabled {
        spawn_world_axes(&mut commands, &mut world_axes)
    }
}

fn spawn_world_axes(commands: &mut Commands, world_axes: &mut ResMut<WorldAxes>) {
    let red = world_axes.standard_materials[0].clone();
    let green = world_axes.standard_materials[1].clone();
    let blue = world_axes.standard_materials[2].clone();

    let axis_mesh = world_axes.axis_mesh.as_ref().unwrap().clone();

    world_axes.axes_entity = Some(
        commands
            .spawn_bundle((
                GlobalTransform::identity(),
                Transform::from_scale(Vec3::splat(world_axes.scale)),
                WorldAxesTag,
            ))
            .with_children(|axes_root| {
                axes_root.spawn_bundle(HUDPbrBundle {
                    material: red.clone(),
                    mesh: axis_mesh.clone(),
                    transform: Transform::from_matrix(Mat4::from_rotation_translation(
                        Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
                        Vec3::new(0.5, 0.0, 0.0),
                    )),
                    ..Default::default()
                });
                axes_root.spawn_bundle(HUDPbrBundle {
                    material: green.clone(),
                    mesh: axis_mesh.clone(),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..Default::default()
                });
                axes_root.spawn_bundle(HUDPbrBundle {
                    material: blue.clone(),
                    mesh: axis_mesh,
                    transform: Transform::from_matrix(Mat4::from_rotation_translation(
                        Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                        Vec3::new(0.0, 0.0, 0.5),
                    )),
                    ..Default::default()
                });
            })
            .id(),
    );
}

fn world_axes_toggle_system(mut commands: Commands, mut world_axes: ResMut<WorldAxes>) {
    if world_axes.enabled {
        if world_axes.axes_entity.is_none() {
            spawn_world_axes(&mut commands, &mut world_axes);
        }
    } else if let Some(entity) = world_axes.axes_entity {
        commands.entity(entity).despawn_recursive();
        world_axes.axes_entity = None;
    }
}

// NOTE: This system depends on the tagged camera's GlobalTransform having been updated!
fn world_axes_system(
    world_axes: Res<WorldAxes>,
    mut queries: QuerySet<(
        Query<(&Camera, &GlobalTransform), With<WorldAxesPositionTag>>,
        Query<&GlobalTransform, (With<Camera>, With<WorldAxesRotationTag>)>,
        Query<&mut Transform, (Without<Camera>, With<WorldAxesTag>)>,
    )>,
) {
    if !world_axes.enabled || world_axes.axes_entity.is_none() {
        return;
    }
    let mut translation = None;
    if let Ok((camera, camera_transform)) = queries.q0().single() {
        let inv_view_matrix = camera_transform.compute_matrix();
        let projection_matrix = camera.projection_matrix;
        let world_pos: Vec4 = (inv_view_matrix * projection_matrix.inverse())
            .mul_vec4(world_axes.clip_space_position.extend(1.0));
        let position: Vec3 = (world_pos / world_pos.w).truncate().into();

        translation = Some(position);
    }
    let mut rotation = None;
    if let Ok(camera_transform) = queries.q1().single() {
        rotation = Some(camera_transform.rotation.inverse());
    }
    if let Ok(mut axes_transform) = queries.q2_mut().single_mut() {
        if let Some(translation) = translation {
            axes_transform.translation = translation;
        }
        if let Some(rotation) = rotation {
            axes_transform.rotation = rotation;
        }
    }
}
