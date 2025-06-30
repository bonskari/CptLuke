use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration { 
            gravity: Vec3::new(0.0, -9.81, 0.0), 
            ..default()
        })
        
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement_system, grab_cursor, interact_with_consoles, update_screen_effect))
        .run();
}

#[derive(Component)]
struct ScreenEffect {
    initial_emissive_color: Color,
    time_elapsed: f32,
}

fn update_screen_effect(
    time: Res<Time>,
    mut query: Query<(&ScreenEffect, &mut Handle<StandardMaterial>)>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (screen_effect, material_handle) in &mut query {
        let material = materials.get_mut(&*material_handle).unwrap();
        let t = (screen_effect.time_elapsed + time.delta_seconds() * 2.0).sin() * 0.5 + 0.5; // Oscillate between 0 and 1
        material.emissive = screen_effect.initial_emissive_color * t * 2.0;
    }
}

fn interact_with_consoles(
    player_query: Query<&Transform, With<Player>>,
    mut interactable_query: Query<(&Transform, &mut Handle<StandardMaterial>), With<Interactable>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_transform = player_query.single();

    for (interactable_transform, mut material_handle) in interactable_query.iter_mut() {
        let distance = player_transform.translation.distance(interactable_transform.translation);

        if distance < 2.0 && keyboard_input.just_pressed(KeyCode::KeyE) {
            // Change color to red when interacted with
            *material_handle = materials.add(StandardMaterial::from(Color::rgb(1.0, 0.0, 0.0)));
        }
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Interactable;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn the player and camera
    commands.spawn((
        Player,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.2, 4.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::capsule_y(0.9, 0.3),
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED,
    ));

    // Add lights
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 800.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 8.0, -4.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..default()
    });

    // Create the floor
    // Create the floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::Y))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("textures/floor_plating.png")),
            perceptual_roughness: 0.8, // Make it less shiny
            metallic: 0.1,
            ..default()
        }),
        ..default()
    })
    .insert(Collider::cuboid(5.0, 0.01, 5.0));

    // Create the main screen
    commands.spawn(PbrBundle {
        mesh: meshes.add(Rectangle::new(6.0, 3.0)),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("textures/space_view.png")),
            emissive: Color::rgb(0.1, 0.2, 0.3) * 10.0, // Subtle glow for space view
            unlit: true, // Space view should not be affected by scene lighting
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 2.0, -4.9),
        ..default()
    })
    .insert(Collider::cuboid(3.0, 1.5, 0.01))
    .insert(ScreenEffect { initial_emissive_color: Color::rgb(0.1, 0.2, 0.3), time_elapsed: 0.0 });

    // Create walls
    // Back wall
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(10.0, 5.0, 0.1))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("textures/wall_paneling.png")),
            perceptual_roughness: 0.8,
            metallic: 0.1,
            
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 2.5, -5.0),
        ..default()
    })
    .insert(Collider::cuboid(5.0, 2.5, 0.05));

    // Front wall
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(10.0, 5.0, 0.1))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("textures/wall_paneling.png")),
            perceptual_roughness: 0.8,
            metallic: 0.1,
            
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 2.5, 5.0),
        ..default()
    })
    .insert(Collider::cuboid(5.0, 2.5, 0.05));

    // Left wall
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(0.1, 5.0, 10.0))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("textures/wall_paneling.png")),
            perceptual_roughness: 0.8,
            metallic: 0.1,
            
            ..default()
        }),
        transform: Transform::from_xyz(-5.0, 2.5, 0.0),
        ..default()
    })
    .insert(Collider::cuboid(0.05, 2.5, 5.0));

    // Right wall
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(0.1, 5.0, 10.0))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("textures/wall_paneling.png")),
            perceptual_roughness: 0.8,
            metallic: 0.1,
            
            ..default()
        }),
        transform: Transform::from_xyz(5.0, 2.5, 0.0),
        ..default()
    })
    .insert(Collider::cuboid(0.05, 2.5, 5.0));

    // Ceiling
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Plane3d::new(Vec3::NEG_Y))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("textures/wall_paneling.png")),
            perceptual_roughness: 0.8,
            metallic: 0.1,
            
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    })
    .insert(Collider::cuboid(5.0, 0.01, 5.0));

    

    // Create consoles
    for i in 0..3 {
        let console_x = -3.0 + (i as f32 * 3.0);

        commands.spawn(SpatialBundle::from_transform(Transform::from_xyz(console_x, 0.0, 2.0)))
            .with_children(|parent| {
                // Console base
                parent.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(Cuboid::new(1.5, 0.6, 0.8))),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("textures/console_casing.png")),
                        perceptual_roughness: 0.8,
                        metallic: 0.1,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.3, 0.0),
                    ..default()
                }).insert(Collider::cuboid(0.75, 0.3, 0.4));

                // Console top part
                parent.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(Cuboid::new(1.3, 0.1, 0.7))),
                    material: materials.add(StandardMaterial::from(Color::rgb(0.3, 0.3, 0.3))),
                    transform: Transform::from_xyz(0.0, 0.65, 0.0),
                    ..default()
                }).insert(Collider::cuboid(0.65, 0.05, 0.35));

                // Console screen
                parent.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(Rectangle::new(1.0, 0.4))),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("textures/screen_interface.png")),
                        emissive: Color::rgb(0.0, 0.8, 0.0) * 5.0, // Brighter greenish glow
                        perceptual_roughness: 0.1,
                        metallic: 0.0,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.7, -0.35),
                    ..default()
                }).insert(Collider::cuboid(0.5, 0.2, 0.01))
                .insert(ScreenEffect { initial_emissive_color: Color::rgb(0.0, 0.8, 0.0), time_elapsed: 0.0 });
            })
            .insert(Interactable);
    }
}

fn player_movement_system(
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    time: Res<Time>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((mut transform, mut velocity)) = player_query.get_single_mut() {
        let window = q_windows.single();
        // Only move if the cursor is locked
        if window.cursor.grab_mode == CursorGrabMode::Locked {
            let mut lin_vel = Vec3::ZERO;
            let forward = -transform.forward();
            let right = transform.right();

            if keyboard_input.pressed(KeyCode::KeyW) {
                lin_vel -= *forward;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                lin_vel += *forward;
            }
            if keyboard_input.pressed(KeyCode::KeyA) {
                lin_vel -= *right;
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                lin_vel += *right;
            }

            // Project the velocity onto the horizontal plane
            lin_vel.y = 0.0;

            velocity.linvel = lin_vel.normalize_or_zero() * 5.0;

            // Mouse look
            let mut delta_x = 0.0;
            let mut delta_y = 0.0;
            for event in mouse_motion_events.read() {
                delta_x += event.delta.x;
                delta_y += event.delta.y;
            }

            // Yaw (left/right)
            let yaw_rotation = Quat::from_rotation_y(-delta_x * 0.1 * time.delta_seconds());
            transform.rotation = yaw_rotation * transform.rotation;

            // Pitch (up/down)
            let pitch_rotation = Quat::from_rotation_x(-delta_y * 0.1 * time.delta_seconds());
            transform.rotation = transform.rotation * pitch_rotation;
        }
    }
}

fn grab_cursor(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = q_windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}