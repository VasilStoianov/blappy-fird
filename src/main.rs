use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, Sprite, SpriteBundle},
    DefaultPlugins,
};

use rand::{thread_rng, Rng};

static mut COLLIDED: &bool = &false;

#[derive(Component, Debug)]
enum Direction {
    Up,
    Down,
}

#[derive(Component)]
struct Player {
    collide: bool,
}

impl Player {
    fn new() -> Self {
        Self { collide: false }
    }
}

#[derive(Resource)]
struct MoveConfig {
    timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1024.,
                height: 720.,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(movement)
        .add_system(tick_timer)
        .add_system(move_player)
        .add_system(check_collision)
        //.add_system(spawn_new_pipe)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    //Rectangle
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 255.0, 0.),
                custom_size: Some(Vec2::new(150., 150.)),
                ..default()
            },
            transform: Transform::from_xyz(50., 150., 0.),

            ..default()
        },
        Direction::Up,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 255.0, 0.),
                custom_size: Some(Vec2::new(150., 150.)),
                ..default()
            },
            transform: Transform::from_xyz(50., -150., 0.),

            ..default()
        },
        Direction::Down,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(255., 0., 0.),
                custom_size: Some(Vec2::new(25., 25.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Player::new(),
    ));

    commands.insert_resource(MoveConfig {
        timer: Timer::new(Duration::from_millis(125), TimerMode::Repeating),
    });
}
fn movement(
    time: Res<MoveConfig>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
    mut commads: Commands,
    asset_server: Res<AssetServer>,
) {
    unsafe {
        if *COLLIDED {
            let game_over = asset_server.load("GameOverText.png");
            commads.spawn(SpriteBundle {
                texture: game_over,
                ..Default::default()
            });
        } else {
            if time.timer.finished() {
                for (mut direction, mut transform) in &mut sprite_position {
                    match *direction {
                        Direction::Up => transform.translation.x -= 10.,
                        Direction::Down => transform.translation.x -= 10.,
                    }
                }
            }
        }
    }
}

fn tick_timer(mut timer: ResMut<MoveConfig>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}

fn spawn_new_pipe(mut commands: Commands, time: Res<MoveConfig>) {
    let mut rnd = thread_rng();
    let r = rnd.gen_range(0.0..255.);
    let g = rnd.gen_range(0.0..255.);

    if time.timer.finished() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(
                        rnd.gen_range(0.0..255.),
                        rnd.gen_range(0.0..255.),
                        rnd.gen_range(0.0..255.),
                    ),
                    custom_size: Some(Vec2::new(-10., 40.)),
                    ..default()
                },
                transform: Transform::from_xyz(r, g, 0.),
                ..default()
            },
            Direction::Up,
        ));
    }
}

fn move_player(
    mut query: Query<(&mut Player, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut player, mut transform) in query.iter_mut() {
        if !player.collide {
            match keyboard_input.pressed(KeyCode::Space) {
                true => transform.translation.y += 25.,
                false => transform.translation.y -= 1.,
            }
        }
    }
}

fn check_collision(
    mut player: Query<(&mut Player, &Transform)>,
    boxes: Query<(&Direction, &Transform)>,
) {
    unsafe {
        for (mut pl, pl_transform) in player.iter_mut() {
            for (pipe, pipe_transform) in boxes.into_iter() {
                if collide(
                    pl_transform.translation,
                    Vec2::new(25., 25.),
                    pipe_transform.translation,
                    Vec2::new(150., 150.),
                )
                .is_some()
                {
                    pl.collide = true;
                    COLLIDED = &true;
                }
            }
        }
    }
}
