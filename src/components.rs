use bevy::prelude::*;

#[derive(Component)]
pub struct RedBall {
    pub collision_count: usize,
}

#[derive(Component)]
pub struct BlueRectangle;

#[derive(Component)]
pub struct Ceil;

#[derive(Component)]
pub struct Floor;

#[derive(Component)]
pub struct LeftWall;

#[derive(Component)]
pub struct RightWall;

#[derive(Component)]
pub struct ScoreText;