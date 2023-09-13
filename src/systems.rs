use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use crate::{
    components::{BlueRectangle, Ceil, Floor, LeftWall, RedBall, RightWall, ScoreText},
    SpawnRedBallTimer, BLUE_RECTANGLE_HEIGHT, BLUE_RECTANGLE_WIDTH, LINEAR_VELOCITY,
    RED_BALL_READIUS, RESTITUTION, WALL_COLOR, WALL_THICKNESS, RED_BALL_COLLISION_COUNT, GameState, Score,
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_walls(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();
    let width = window.width();
    let height = window.height();
    commands.spawn(wall(
        width,
        WALL_THICKNESS,
        0.,
        height / 2. - WALL_THICKNESS / 2.,
        Ceil,
    ));
    commands.spawn(wall(
        width,
        WALL_THICKNESS,
        0.,
        -height / 2. + WALL_THICKNESS / 2.,
        Floor,
    ));
    commands.spawn(wall(
        WALL_THICKNESS,
        height - 2. * WALL_THICKNESS,
        -width / 2. + WALL_THICKNESS / 2.,
        0.,
        LeftWall,
    ));
    commands.spawn(wall(
        WALL_THICKNESS,
        height - 2. * WALL_THICKNESS,
        width / 2. - WALL_THICKNESS / 2.,
        0.,
        RightWall,
    ));
}

fn wall<T: Component>(
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    marker: T,
) -> (SpriteBundle, RigidBody, Position, Restitution, Collider, T) {
    (
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.),
            ..default()
        },
        RigidBody::Static,
        Position(Vec2::new(x, y)),
        Restitution::new(RESTITUTION),
        Collider::cuboid(width, height),
        marker,
    )
}

pub fn tick_to_spawn_red_ball(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    mut mesh: ResMut<Assets<Mesh>>,
    mut color_material: ResMut<Assets<ColorMaterial>>,
    mut timer: ResMut<SpawnRedBallTimer>,
    time: Res<Time>,
) {
    if timer.timer.tick(time.delta()).finished() {
        let window = window.get_single().unwrap();
        let width = window.width();
        let height = window.height();
        let x = rand::thread_rng().gen_range(
            (-width / 2. + WALL_THICKNESS + RED_BALL_READIUS + 10.)
                ..=(width / 2. - WALL_THICKNESS - RED_BALL_READIUS - 10.),
        );
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: mesh
                    .add(
                        shape::Circle {
                            radius: RED_BALL_READIUS,
                            ..default()
                        }
                        .into(),
                    )
                    .into(),
                material: color_material.add(Color::RED.into()),
                transform: Transform::from_xyz(
                    x,
                    height / 2. - WALL_THICKNESS - RED_BALL_READIUS - 10.,
                    0.,
                ),
                ..default()
            },
            RedBall { collision_count: 0 },
            RigidBody::Dynamic,
            Position(Vec2::new(
                x,
                height / 2. - WALL_THICKNESS - RED_BALL_READIUS - 10.,
            )),
            Restitution::new(RESTITUTION),
            Collider::ball(RED_BALL_READIUS),
        ));
    }
}

pub fn spawn_blue_rectangle(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();
    let height = window.height();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(0x00, 0xf5, 0xff),
                custom_size: Some(Vec2::new(BLUE_RECTANGLE_WIDTH, BLUE_RECTANGLE_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0., -height / 2. + WALL_THICKNESS + 20., 0.0),
            ..default()
        },
        BlueRectangle,
        RigidBody::Kinematic,
        Position(Vec2::new(0., -height / 2. + WALL_THICKNESS + 20.)),
        Restitution::new(RESTITUTION),
        Collider::cuboid(BLUE_RECTANGLE_WIDTH, BLUE_RECTANGLE_HEIGHT),
    ));
}

pub fn spawn_score_text(
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
        background_color: Color::rgba_u8(0, 0, 0, 0).into(),
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexStart,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section("Score: 0", TextStyle { font_size: 50., color: Color::RED, ..default()}),
            ScoreText,
        ));
    });
}

pub fn blue_rectangle_movement(
    mut player: Query<(&mut LinearVelocity, &CollidingEntities), With<BlueRectangle>>,
    left_wall: Query<Entity, With<LeftWall>>,
    right_wall: Query<Entity, With<RightWall>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok((mut velocity, colliding_entities)) = player.get_single_mut() {
        if let Ok(left_wall_entity) = left_wall.get_single() {
            if let Ok(right_wall_entity) = right_wall.get_single() {
                if keyboard_input.pressed(KeyCode::Left)
                    && !colliding_entities.contains(&left_wall_entity)
                {
                    velocity.x = -LINEAR_VELOCITY;
                } else if keyboard_input.pressed(KeyCode::Right)
                    && !colliding_entities.contains(&right_wall_entity)
                {
                    velocity.x = LINEAR_VELOCITY;
                } else if (velocity.x - 0.).abs() > f32::EPSILON {
                    velocity.x = 0.;
                }
            }
        }
    }
}

pub fn bouncing_red_ball(
    mut commands: Commands,
    blue_rectangle: Query<Entity, With<BlueRectangle>>,
    mut red_balls: Query<(Entity, &mut LinearVelocity, &mut RedBall), With<RedBall>>,
    mut score_text: Query<&mut Text, With<ScoreText>>,
    mut collisions: EventReader<CollisionStarted>,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
) {
    for CollisionStarted(entity1, entity2) in collisions.iter() {
        commands.spawn(AudioBundle {
            source: asset_server.load("pluck.ogg"),
            settings: PlaybackSettings::DESPAWN,
        });
        if let Ok(blue_entity) = blue_rectangle.get_single() {
            for (red_entity, mut velocity, mut red_ball) in red_balls.iter_mut() {
                if (blue_entity == *entity1 || blue_entity == *entity2)
                    && (red_entity == *entity1 || red_entity == *entity2)
                {
                    red_ball.collision_count += 1;
                    velocity.y += 250.;
                    if red_ball.collision_count >= RED_BALL_COLLISION_COUNT {
                        score.score += 1;
                        if let Ok(mut text) = score_text.get_single_mut() {
                            text.sections[0].value = format!("Score: {}", score.score);
                        }
                        commands.entity(red_entity).despawn();
                    }
                }
            }
        }
    }
}

pub fn despawn_red_ball(
    mut commands: Commands,
    floor: Query<Entity, With<Floor>>,
    red_balls: Query<Entity, With<RedBall>>,
    mut collisions: EventReader<CollisionStarted>,
) {
    for CollisionStarted(entity1, entity2) in collisions.iter() {
        if let Ok(floor_entity) = floor.get_single() {
            for red_entity in red_balls.iter() {
                if (floor_entity == *entity1 || floor_entity == *entity2)
                    && (red_entity == *entity1 || red_entity == *entity2)
                {
                    commands.entity(red_entity).despawn();
                }
            }
        }
    }
}

pub fn toggle_game_state(
    current_state: Res<State<GameState>>,
    mut game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let new_state = match current_state.get() {
            GameState::Running => GameState::Paused,
            GameState::Paused => GameState::Running,
        };
        game_state.set(new_state);
    }
}
