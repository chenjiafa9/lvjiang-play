use bevy::prelude::*;

#[derive(Component)]
struct Plary;

pub struct SetupPlugin;

fn plary_run(mut commands: Commands, mut plary: Query<&Plary>) {
    let play = plary.single();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let sxd: Handle<Scene> = asset_server.load("other_test/测试人.glb#Scene0");

    commands.spawn((
        Plary,
        SceneBundle {
            scene: sxd,
            ..default()
        },
    ));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(40.0, 5.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(-20.0, 20.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PbrBundle {
        transform:Transform::from_xyz(0.0, -15.0, 0.0),
        mesh: meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
