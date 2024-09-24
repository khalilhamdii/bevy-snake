use crate::components::position::Position;
use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct LastTailPosition(pub Option<Position>);
