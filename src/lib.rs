use bevy_flycam::FlyCam;
use bevy_infinite_grid::InfiniteGridBundle;
use js_sys::wasm_bindgen::prelude::*;
use bevy::{pbr::wireframe::{ Wireframe, WireframeColor}, prelude::*};
fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let terrian = Mesh::from(shape::Plane {
        size: 10.,
        subdivisions: 5,
    });
    
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(terrian),

            material: materials.add(Color::WHITE.into()),
            ..default()
        },
        Wireframe,
        WireframeColor { color: Color::RED },
    ));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10000.,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 6., 12.0)
                .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        },
        FlyCam,
    ));
    commands.spawn(InfiniteGridBundle::default());
}
#[wasm_bindgen]
pub fn run_bevy_app() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // provide the ID selector string here
            canvas: Some("#mygame-canvas".into()),
            ..default()
        }),
    
        ..default()
    }))
        .add_systems(Startup, start)
        .run();
}