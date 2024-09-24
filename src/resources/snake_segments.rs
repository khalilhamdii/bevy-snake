use bevy::prelude::{Entity, Resource};

#[derive(Resource, Default)]
pub struct SnakeSegments(pub Vec<Entity>);
