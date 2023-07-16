use bevy::{
    prelude::*,
    sprite::{collide_aabb::*, MaterialMesh2dBundle},
};
use rand::{thread_rng, Rng};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, cheek_collision)
        .add_systems(Update, update)
        .run();
}
#[derive(Component, Default)]
struct Collider;
#[derive(Bundle, Default)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}
impl WallBundle {
    fn new(translation: Vec3, scale: Vec3) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation,
                    scale,
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::WHITE,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
fn wall_loading(commands: &mut Commands, position: Vec3, size: Vec2) {
    const WIDE: f32 = 10.;
    commands.spawn(WallBundle::new(
        Vec3::new(position.x, position.y + size.y / 2., position.z),
        Vec3::new(size.x + WIDE / 2., WIDE, 0.),
    ));
    commands.spawn(WallBundle::new(
        Vec3::new(position.x, position.y - size.y / 2., position.z),
        Vec3::new(size.x - WIDE / 2., WIDE, 0.),
    ));
    commands.spawn(WallBundle::new(
        Vec3::new(position.x - size.x / 2., position.y, position.z),
        Vec3::new(WIDE, size.y + WIDE, 0.),
    ));
    commands.spawn(WallBundle::new(
        Vec3::new(position.x + size.x / 2., position.y, position.z),
        Vec3::new(WIDE, size.y + WIDE, 0.),
    ));
}

#[derive(Component)]
struct Ball(Vec2);
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    wall_loading(&mut commands, Vec3::default(), Vec2::new(1250., 700.));
    for i in 0..10 {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(0.8).into()).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(
                    (thread_rng().gen_range(0..100) as f32) / 100.,
                    (thread_rng().gen_range(0..100) as f32) / 100.,
                    (thread_rng().gen_range(0..100) as f32) / 100.,
                ))),
                transform: Transform {
                    translation: Vec3::new(
                        thread_rng().gen_range(-200..200) as f32,
                        thread_rng().gen_range(-200..200) as f32,
                        i as f32,
                    ),
                    scale: Vec3::splat(thread_rng().gen_range(50..100) as f32),
                    ..Default::default()
                },
                ..default()
            },
            Ball(Vec2::new(
                thread_rng().gen_range(-20..20) as f32,
                thread_rng().gen_range(-20..20) as f32,
            )),
        ));
    }
}

fn cheek_collision(
    mut ball_positoin: Query<(&mut Ball, &Transform), With<Ball>>,
    wall_positoin: Query<&Transform, With<Collider>>,
) {
    for (mut ball, ball_transform) in &mut ball_positoin {
        for wall_transform in &wall_positoin {
            if let Some(value) = collide(
                ball_transform.translation,
                ball_transform.scale.truncate(),
                wall_transform.translation,
                wall_transform.scale.truncate(),
            ) {
                match value {
                    Collision::Left | Collision::Right => ball.0.x = -ball.0.x,
                    Collision::Top | Collision::Bottom => ball.0.y = -ball.0.y,
                    Collision::Inside => (),
                }
            }
        }
    }
}

fn update(mut ball_positoin: Query<(&Ball, &mut Transform), With<Ball>>) {
    for (ball, mut transform) in &mut ball_positoin {
        transform.translation.x += ball.0.x;
        transform.translation.y += ball.0.y;
    }
}
