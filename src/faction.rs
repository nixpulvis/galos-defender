use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Component, Debug)]
pub(crate) struct Faction {
    pub name: String,
}

#[derive(Component, Debug)]
pub(crate) struct Systems(pub HashSet<Entity>);
