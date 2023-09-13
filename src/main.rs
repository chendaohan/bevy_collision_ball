use bevy::prelude::*;
use bevy_xpbd_2d::{
    prelude::{setup, PhysicsPlugins},
    resources::{Gravity, SubstepCount},
};
use systems::{
    blue_rectangle_movement, bouncing_red_ball, despawn_red_ball, spawn_blue_rectangle,
    spawn_camera, spawn_walls, tick_to_spawn_red_ball, toggle_game_state, spawn_score_text,
};

pub mod components;
pub mod systems;

pub const RED_BALL_READIUS: f32 = 20.;
pub const RED_BALL_COLLISION_COUNT: usize = 2;
pub const RED_BALL_SPAWN_TIME: f32 = 1.;
pub const BLUE_RECTANGLE_WIDTH: f32 = 350.;
pub const BLUE_RECTANGLE_HEIGHT: f32 = 20.;
pub const WALL_THICKNESS: f32 = 15.;
pub const WALL_COLOR: Color = Color::MIDNIGHT_BLUE;
pub const RESTITUTION: f32 = 0.7;
pub const LINEAR_VELOCITY: f32 = 800.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .init_resource::<SpawnRedBallTimer>()
        .init_resource::<Score>()
        .insert_resource(Gravity(Vec2::NEG_Y * 40.))
        .insert_resource(SubstepCount(6))
        .insert_resource(ClearColor(Color::GRAY))
        .insert_resource(GlobalVolume::new(0.4))
        .add_state::<GameState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, (spawn_walls, spawn_blue_rectangle, spawn_score_text))
        .add_systems(OnEnter(GameState::Running), setup::resume)
        .add_systems(OnEnter(GameState::Paused), setup::pause)
        .add_systems(Update, toggle_game_state)
        .add_systems(
            Update,
            (
                tick_to_spawn_red_ball,
                blue_rectangle_movement,
                bouncing_red_ball,
                despawn_red_ball,
            )
                .run_if(in_state(GameState::Running)),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}

#[derive(Resource)]
pub struct SpawnRedBallTimer {
    timer: Timer,
}

impl Default for SpawnRedBallTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(RED_BALL_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource, Default)]
pub struct Score {
    score: usize,
}
