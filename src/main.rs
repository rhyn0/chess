mod models;
use bevy::{
    prelude::*,
    window::{PresentMode, Window, WindowMode, WindowPlugin, WindowPosition},
};
use bevy_mod_picking::prelude::{DefaultPickingPlugins, PickableBundle};

use models::pieces;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess".to_string(),
                mode: WindowMode::Windowed,
                position: WindowPosition::Automatic,
                resolution: (1200., 800.).into(),
                present_mode: PresentMode::AutoNoVsync,
                fit_canvas_to_parent: true,
                resizable: true,
                focused: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .insert_resource(Msaa::default()) // default of this is 4 samples
        .add_systems(Startup, (setup, spawn_board, pieces::spawn_pieces_on_board))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        },
        PickableBundle::default(),
    ));
    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}

fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane::from_size(1.0)));
    let white_material = materials.add(Color::WHITE.into());
    let black_material = materials.add(Color::BLACK.into());
    // spawn 64 alternating squares
    for i in 0i8..8 {
        for j in 0i8..8 {
            let (i_float, j_float): (f32, f32) = (f32::from(i), f32::from(j));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: if (i + j + 1) % 2 == 0 {
                        // fills from bottom left
                        white_material.clone()
                    } else {
                        black_material.clone()
                    },
                    transform: Transform::from_translation(Vec3::new(i_float, 0., j_float)),
                    ..default()
                },
                PickableBundle::default(),
            ));
        }
    }
}
