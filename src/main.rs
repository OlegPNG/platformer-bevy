use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default() 
        }))
        .add_plugin(EditorPlugin::default())
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        //.add_startup_system(zoom_camera.after(setup))
        .insert_resource(LevelSelection::Index(0))
        .run();
}

#[derive(Component)]
struct MyGameCamera;

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        //Camera2dBundle {
        //    transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        //    ..default()
        //},
        Camera2dBundle::default(),
        MyGameCamera,
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("maps/ldtk-test.ldtk"),
        ..Default::default()
    });
}

//fn zoom_camera(
//    mut camera_query: Query<&mut OrthographicProjection, With<Camera2d>>,
//) {
//    let mut projection = camera_query.single_mut();
//
//    projection.scale = 0.5;
//
//}
