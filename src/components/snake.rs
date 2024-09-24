use crate::utilities::Direction;
use bevy::prelude::Component;

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
}

#[derive(Component)]
pub struct SnakeSegment;
