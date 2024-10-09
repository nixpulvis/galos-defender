use bevy::prelude::*;

#[derive(Bundle, Debug)]
pub(crate) struct SystemBundle {
    pub system: System,
    pub position: Position,
}

#[derive(Component, Debug)]
pub(crate) struct System {
    pub name: String,
    pub address: u32,
}

#[derive(Component, Debug)]
pub(crate) struct Position(pub Vec3);
