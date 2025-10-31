use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI};

use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, SamplerDescriptor};
use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin, FreeCameraState},
    color::palettes::tailwind,
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    math::Affine2,
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(FreeCameraPlugin)
        // Example code plugins
        .add_plugins((CameraPlugin, CameraSettingsPlugin, ScenePlugin))
        .add_plugins(
            DefaultPlugins.set(ImagePlugin {
                default_sampler: SamplerDescriptor {
                    address_mode_u: AddressMode::Repeat,
                    address_mode_v: AddressMode::Repeat,
                    address_mode_w: AddressMode::Repeat,
                    ..Default::default()
                }
                .into(),
            }),
        )
        .run();
}

// Plugin that spawns the camera.
struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 0.0).looking_to(Vec3::X, Vec3::Y),
        // This component stores all camera settings and state, which is used by the FreeCameraPlugin to
        // control it. These properties can be changed at runtime, but beware the controller system is
        // constantly using and modifying those values unless the enabled field is false.
        FreeCamera {
            sensitivity: 0.2,
            friction: 25.0,
            walk_speed: 3.0,
            run_speed: 9.0,
            ..default()
        },
    ));
}

// Plugin that handles camera settings controls and information text
struct CameraSettingsPlugin;
impl Plugin for CameraSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_text)
            .add_systems(Update, (update_camera_settings, update_text));
    }
}

#[derive(Component)]
struct InfoText;

fn spawn_text(mut commands: Commands, free_camera_query: Query<&FreeCamera>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(-16),
            left: px(12),
            ..default()
        },
        children![Text::new(format!(
            "{}",
            free_camera_query.single().unwrap()
        ))],
    ));
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
        children![Text::new(concat![
            "Z/X: decrease/increase sensitivity\n",
            "C/V: decrease/increase friction\n",
            "F/G: decrease/increase scroll factor\n",
            "B: enable/disable controller",
        ]),],
    ));

    // Mutable text marked with component
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            right: px(12),
            ..default()
        },
        children![(InfoText, Text::new(""))],
    ));
}

fn update_camera_settings(
    mut camera_query: Query<(&mut FreeCamera, &mut FreeCameraState)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (mut free_camera, mut free_camera_state) = camera_query.single_mut().unwrap();

    if input.pressed(KeyCode::KeyZ) {
        free_camera.sensitivity = (free_camera.sensitivity - 0.005).max(0.005);
    }
    if input.pressed(KeyCode::KeyX) {
        free_camera.sensitivity += 0.005;
    }
    if input.pressed(KeyCode::KeyC) {
        free_camera.friction = (free_camera.friction - 0.2).max(0.0);
    }
    if input.pressed(KeyCode::KeyV) {
        free_camera.friction += 0.2;
    }
    if input.pressed(KeyCode::KeyF) {
        free_camera.scroll_factor = (free_camera.scroll_factor - 0.02).max(0.02);
    }
    if input.pressed(KeyCode::KeyG) {
        free_camera.scroll_factor += 0.02;
    }
    if input.just_pressed(KeyCode::KeyB) {
        free_camera_state.enabled = !free_camera_state.enabled;
    }
}

fn update_text(
    mut text_query: Query<&mut Text, With<InfoText>>,
    camera_query: Query<(&FreeCamera, &FreeCameraState)>,
) {
    let mut text = text_query.single_mut().unwrap();

    let (free_camera, free_camera_state) = camera_query.single().unwrap();

    text.0 = format!(
        "Enabled: {},\nSensitivity: {:.03}\nFriction: {:.01}\nScroll factor: {:.02}\nWalk Speed: {:.02}\nRun Speed: {:.02}\nSpeed: {:.02}",
        free_camera_state.enabled,
        free_camera.sensitivity,
        free_camera.friction,
        free_camera.scroll_factor,
        free_camera.walk_speed,
        free_camera.run_speed,
        free_camera_state.velocity.length(),
    );
}

// Plugin that spawns the scene and lighting.
struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_lights, spawn_world));
    }
}

fn spawn_lights(mut commands: Commands) {
    // Main light
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::NEUTRAL_300),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 45.0, 0.0),
    ));
}

fn spawn_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let floor = meshes.add(Plane3d::new(
        Vec3::new(0.0, 100.0, 0.0),
        Vec2::new(20.0, 35.0),
    ));
    let sphere = meshes.add(Sphere::new(0.5));

    let wall = meshes.add(Cuboid::new(0.2, 4.0, 3.0));
    let back_wall = meshes.add(Cuboid::new(50.0, 5.0, 0.35));
    let cub_wall = meshes.add(Cuboid::new(5.0, 5.0, 0.2));
    let tav_wall = meshes.add(Cuboid::new(9.0, 5.0, 0.35));

    let long_wall = meshes.add(Cuboid::new(80.0, 5.0, 0.35));
    let cub_ent = meshes.add(Cuboid::new(2.0, 5.0, 0.15));
    let shor_ent = meshes.add(Cuboid::new(1.0, 5.0, 0.15));

    let hall_1 = meshes.add(Cuboid::new(5.0, 5.0, 0.15));

    let column = meshes.add(Cylinder::new(0.3, 5.0));
    let blue_material = materials.add(Color::from(tailwind::BLUE_700));
    let red_material = materials.add(Color::from(tailwind::RED_950));
    let white_material = materials.add(Color::WHITE);
    let texture_handle = asset_server.load("textures/marble.png");
    let skyeee = asset_server.load("textures/skybox.png");
    let floa = asset_server.load("textures/floor.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let flooo = materials.add(StandardMaterial {
        base_color_texture: Some(floa.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let skybox = materials.add(StandardMaterial {
        base_color_texture: Some(skyeee.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let sky = meshes.add(Circle::new(100.0));
    // Top side of floor

    commands.spawn((
        Mesh3d(floor.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load_with_settings(
                "textures/floor.png",
                |s: &mut _| {
                    *s = ImageLoaderSettings {
                        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                            // rewriting mode to repeat image,
                            address_mode_u: ImageAddressMode::MirrorRepeat,
                            address_mode_v: ImageAddressMode::MirrorRepeat,

                            ..default()
                        }),

                        ..default()
                    }
                },
            )),
            emissive: LinearRgba::rgb(0.244, 0.166, 0.172),
            // uv_transform used here for proportions only, but it is full Affine2
            // that's why you can use rotation and shift also
            uv_transform: Affine2::from_scale(Vec2::new(20., 20.)),
            ..default()
        })),
    ));

    // Tall wall
    commands.spawn((
        Mesh3d(wall.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform::from_xyz(-3.0, 2.0, 0.0),
    ));
    commands.spawn((
        Mesh3d(long_wall.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform {
            translation: Vec3::new(20.0, 0.0, 0.0),
            rotation: Quat::from_euler(EulerRot::YXZEx, FRAC_PI_2, 0.0, 0.0),
            ..default()
        },
    ));
    commands.spawn((
        Mesh3d(long_wall.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_euler(EulerRot::YXZEx, FRAC_PI_2, 0.0, 0.0),
            ..default()
        },
    ));
    commands.spawn((
        Mesh3d(back_wall.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform::from_xyz(0.0, 0.0, 35.0),
    ));

    commands.spawn((
        Mesh3d(cub_wall.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform::from_xyz(18.0, 0.0, 27.0),
    ));
    commands.spawn((
        Mesh3d(cub_wall.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform::from_xyz(18.0, 0.0, 23.0),
    ));
    commands.spawn((
        Mesh3d(cub_wall.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform::from_xyz(18.0, 0.0, 20.0),
    ));
    commands.spawn((
        Mesh3d(cub_wall.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform::from_xyz(18.0, 0.0, 16.0),
    ));

    commands.spawn((
        Mesh3d(cub_wall.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform::from_xyz(18.0, 0.0, 14.0),
    ));
    commands.spawn((
        Mesh3d(cub_wall.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform::from_xyz(18.0, 0.0, 9.0),
    ));
    commands.spawn((
        Mesh3d(cub_wall.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform::from_xyz(18.0, 0.0, 5.0),
    ));

    commands.spawn((
        Mesh3d(cub_ent.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform {
            translation: Vec3::new(15.5, 0.0, 26.0),
            rotation: Quat::from_euler(EulerRot::YXZEx, FRAC_PI_2, 0.0, 0.0),
            ..default()
        },
    ));
    commands.spawn((
        Mesh3d(cub_ent.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform {
            translation: Vec3::new(15.5, 0.0, 23.0),
            rotation: Quat::from_euler(EulerRot::YXZEx, FRAC_PI_2, 0.0, 0.0),
            ..default()
        },
    ));
    commands.spawn((
        Mesh3d(cub_ent.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform {
            translation: Vec3::new(15.5, 0.0, 20.0),
            rotation: Quat::from_euler(EulerRot::YXZEx, FRAC_PI_2, 0.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        Mesh3d(hall_1.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform {
            translation: Vec3::new(15.5, 0.0, 11.5),
            rotation: Quat::from_euler(EulerRot::YXZEx, FRAC_PI_2, 0.0, 0.0),
            ..default()
        },
    ));
    commands.spawn((
        Mesh3d(shor_ent.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform {
            translation: Vec3::new(15.5, 0.0, 13.0),
            rotation: Quat::from_euler(EulerRot::YXZEx, FRAC_PI_2, 0.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        Mesh3d(tav_wall.clone()),
        MeshMaterial3d(white_material.clone()),
        Transform::from_xyz(16.0, 0.0, 0.0),
    ));

    commands.spawn((
        Mesh3d(column.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(13.0, 0.0, 25.0),
    ));
    commands.spawn((
        Mesh3d(column.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(13.0, 0.0, 24.0),
    ));
    commands.spawn((
        Mesh3d(column.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(13.0, 0.0, 23.0),
    ));
    commands.spawn((
        Mesh3d(column.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(13.0, 0.0, 22.0),
    ));

    commands.spawn((
        Mesh3d(column.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(12.0, 0.0, 25.0),
    ));

    commands.spawn((
        Mesh3d(column.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(11.0, 0.0, 25.0),
    ));

    commands.spawn((
        Mesh3d(column.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(10.0, 0.0, 25.0),
    ));

    commands.spawn((
        Mesh3d(column.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(9.0, 0.0, 25.0),
    ));

    commands.spawn((
        Mesh3d(column.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(9.0, 0.0, 24.0),
    ));
    commands.spawn((
        Mesh3d(column.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(9.0, 0.0, 23.0),
    ));
    commands.spawn((
        Mesh3d(column.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(9.0, 0.0, 22.0),
    ));

    commands.spawn((
        Mesh3d(column.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(12.0, 0.0, 24.0),
    ));

    commands.spawn((
        Mesh3d(column.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(11.0, 0.0, 23.0),
    ));

    commands.spawn((
        Mesh3d(column.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(10.0, 0.0, 22.0),
    ));

    commands.spawn((
        Mesh3d(sky.clone()),
        MeshMaterial3d(skybox.clone()),
        Transform {
            translation: Vec3::new(0.0, 65.0, 0.0),
            rotation: Quat::from_euler(EulerRot::YXZEx, 0.0, FRAC_PI_2, 0.0),
            ..default()
        },
    ));
}
