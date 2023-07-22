use bevy::prelude::*;

const SCALE: f32 = 3.0;
const SCALE_TRANSFORM: Transform = Transform::from_scale(Vec3::new(SCALE, SCALE, SCALE));
const KEYS_LEFT: [KeyCode; 2] = [KeyCode::A, KeyCode::Left];
const KEYS_RIGHT: [KeyCode; 2] = [KeyCode::D, KeyCode::Right];

enum GameStage {
    Grass,
    Snow,
    Castle
}

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    transform: Transform,
}

#[derive(Component)]
struct PlayerMotorcycle;
#[derive(Component, Default)]
struct PlayerKnight {
    sway: f32,
    max_sway: f32,
    sway_gain: f32,
    sway_decay: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_motorcycle,
                update_knight_sway,
                move_knight,
                move_player,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(PlayerBundle {
        player: Player,
        transform: SCALE_TRANSFORM,
    });

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("motorcycle.png"),
            transform: SCALE_TRANSFORM,
            ..Default::default()
        })
        .insert(PlayerMotorcycle);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("motorcycle_player.png"),
            transform: SCALE_TRANSFORM,
            ..Default::default()
        })
        .insert(PlayerKnight {
            sway: 0.0,
            max_sway: 8.0,
            sway_gain: 32.0,
            sway_decay: 0.5,
        });

    for i in 0..8 {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("road.png"),
            transform: SCALE_TRANSFORM.with_translation(Vec3::NEG_Y * 16.0 * SCALE * i as f32),
            ..Default::default()
        });
    }
}

fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = player.get_single_mut().expect("Couldn't find player");
    let dir = player_transform.local_x();

    if keys.any_pressed(KEYS_LEFT) {
        player_transform.translation -= dir * 256.0 * time.delta_seconds();
    }

    if keys.any_pressed(KEYS_RIGHT) {
        player_transform.translation += dir * 256.0 * time.delta_seconds();
    }
}

fn move_motorcycle(
    mut motorcycle: Query<&mut Transform, (With<PlayerMotorcycle>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<PlayerMotorcycle>)>,
) {
    let player_transform = player.get_single().expect("Couldn't find player");
    let mut motorcycle_transform = motorcycle
        .get_single_mut()
        .expect("Couldn't find motorcycle");
    *motorcycle_transform = *player_transform;
}

fn move_knight(
    mut knight: Query<(&mut Transform, &PlayerKnight), Without<Player>>,
    player: Query<&Transform, With<Player>>,
) {
    let player_transform = player.get_single().expect("Couldn't find player");
    let (mut knight_transform, knight_state) =
        knight.get_single_mut().expect("Couldn't find motorcycle");
    let transform = player_transform.with_translation(
        player_transform.translation
            + player_transform.local_x() * knight_state.max_sway * knight_state.sway,
    );
    *knight_transform = transform;
}

fn update_knight_sway(
    mut knight: Query<&mut PlayerKnight>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut knight_state = knight.get_single_mut().expect("Couldn't find motorcycle");

    let left = keys.any_pressed(KEYS_LEFT);
    let right = keys.any_pressed(KEYS_RIGHT);

    if left {
        knight_state.sway =
            (knight_state.sway - knight_state.sway_gain * time.delta_seconds()).max(-1.0);
    }

    if right {
        knight_state.sway =
            (knight_state.sway + knight_state.sway_gain * time.delta_seconds()).min(1.0);
    }

    if !left && !right {
        knight_state.sway *= knight_state.sway_decay;
    }
}
