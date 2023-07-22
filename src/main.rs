use bevy::prelude::*;

/*enum Stage {
    Title,
    Ingame,
    Dead,
}*/

const SCALE: Vec3 = Vec3::new(3.0, 3.0, 3.0);

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Projectile {
    speed: f32
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, shoot, move_projectile))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("motorcycle_player.png"),
            transform: Transform::from_scale(SCALE),
            ..Default::default()
        })
        .insert(Player { speed: 128.0 });
}

fn shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
    player: Query<&Transform, With<Player>>,
) {
    let t = player.get_single().expect("Couldn't find player");
    if keys.just_pressed(KeyCode::Space) {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("knife.png"),
            transform: Transform::from_translation(t.translation).with_scale(SCALE),
            ..Default::default()
        })
        .insert(Projectile {
            speed: 256.0
        });
    }
}

fn move_projectile(
    time: Res<Time>,
    mut projectiles: Query<(&Projectile, &mut Transform)>
) {
    for (p, mut t) in &mut projectiles {
        t.translation.y += p.speed * time.delta_seconds();
    }
}

fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player: Query<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = player.get_single_mut().expect("Couldn't find player");

    if keys.pressed(KeyCode::A) {
        transform.translation.x -= player.speed * time.delta_seconds();
    }

    if keys.pressed(KeyCode::D) {
        transform.translation.x += player.speed * time.delta_seconds();
    }
}
