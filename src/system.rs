use std::collections::HashSet;
use crate::Name;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub(crate) struct System;

#[derive(Component, Debug)]
pub(crate) struct Factions(pub HashSet<Entity>);

#[derive(Component, Debug)]
pub(crate) struct Address(pub u32);

#[derive(Component, Debug)]
pub(crate) struct Position(pub Vec3);

#[derive(Bundle, Debug)]
pub(crate) struct SystemBundle {
    pub system: System,
    pub address: Address,
    pub name: Name,
    pub position: Position,
}
