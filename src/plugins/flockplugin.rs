use crate::plugins::flock::*;
use bevy::{prelude::*, transform::transform_propagate_system};
use rand::prelude::*;

pub struct FlockPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum FlockPluginSystems {
    Setup,
}

impl FlockPlugin {
    fn setup(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>,
        win: Res<Windows>,
    ) {
        let mut rng = rand::thread_rng();

        let wbounds = win.get_primary().unwrap();
        let (hwidth, hheight) = (wbounds.width() / 2f32, wbounds.height() / 2f32);
        let texture = asset_server.load("textures/Acutal_Kishan.png");

        println!("Width Height; {}, {}", hwidth, hheight);
        commands
            .spawn()
            .insert(Flock {
                ..Default::default()
            })
            .with_children(|flock| {
                for _i in 1..200 {
                    flock
                        .spawn()
                        .insert(Boid)
                        .insert_bundle(SpriteBundle {
                            material: materials.add(ColorMaterial {
                                color: Color::WHITE,
                                texture: Some(texture.clone()),
                            }),
                            sprite: Sprite::new(Vec2::new(20.0, 12.0)),
                            ..Default::default()
                        })
                        .insert(Velocity(
                            50.0 * Vec3::new(
                                rng.gen_range(-1.0..=1.0),
                                rng.gen_range(-1.0..=1.0),
                                0.0,
                            ),
                        ))
                        .insert(GlobalTransform::from_translation(Vec3::new(
                            // 0.0, 0.0,
                            rng.gen_range(-hwidth..=hwidth),
                            rng.gen_range(-hheight..=hheight),
                            0.0,
                        )));
                }
            });
    }
}

impl Plugin for FlockPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::setup.system().label(FlockPluginSystems::Setup))
            .add_system(Flock::update.system().label(FlockSystems::Update))
            .add_system(
                Flock::movement
                    .system()
                    .label(FlockSystems::Movement)
                    .after(FlockSystems::Update),
            );
    }
}
