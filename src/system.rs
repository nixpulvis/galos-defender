use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Component, Debug)]
pub(crate) struct System {
    pub name: String,
    pub address: u32,
}

#[derive(Component, Debug)]
pub(crate) struct Factions(pub HashSet<Entity>);

#[derive(Component, Debug)]
pub(crate) struct Position(pub Vec3);
