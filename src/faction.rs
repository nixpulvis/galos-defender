use std::collections::HashSet;
use crate::Name;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub(crate) struct Faction;

#[derive(Component, Debug)]
pub(crate) struct Systems(pub HashSet<Entity>);

#[derive(Bundle, Debug)]
pub(crate) struct FactionBundle {
    pub faction: Faction,
    pub name: Name,
}
