use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Player::spawn.system().label(PlayerSystems::Spawn))
            .add_system(Player::update.system().label(PlayerSystems::Update));
    }
}

pub const PLAYER_SPEED: f32 = 100.0;
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    _p: Player,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum PlayerSystems {
    Spawn,
    Update,
}

impl Player {
    fn spawn(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        assets_server: Res<AssetServer>,
    ) {
        println!("Rofl bob spawned");
        commands
            .spawn_bundle(PlayerBundle { _p: Player })
            .insert_bundle(SpriteBundle {
                material: materials.add(assets_server.load("textures/Acutal_Kishan.png").into()),
                sprite: Sprite::new(Vec2::splat(30.0)),
                ..Default::default()
            });
    }

    fn update(
        time: Res<Time>,
        keys: Res<Input<KeyCode>>,
        mut query: Query<(&Player, &mut Transform)>,
    ) {
        let (_player, mut transform) = query.single_mut().expect("There should only be 1 player");
        let delta = time.delta().as_secs_f32();
        let mut dir = Vec3::ZERO;

        if keys.pressed(KeyCode::D) {
            dir.x += 1.0;
        }
        if keys.pressed(KeyCode::A) {
            dir.x -= 1.0;
        }
        if keys.pressed(KeyCode::W) {
            dir.y += 1.0;
        }
        if keys.pressed(KeyCode::S) {
            dir.y -= 1.0;
        }
        if dir.length_squared() != 0.0 {
            transform.translation += dir * delta * PLAYER_SPEED;
            // println!("{:?}", transform.translation);
        }
    }
}
