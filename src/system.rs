use bevy::prelude::*;

#[derive(Component, Debug)]
pub(crate) struct System {
    pub name: String,
    pub address: u32,
}

#[derive(Component, Debug)]
pub(crate) struct Position(pub Vec3);
