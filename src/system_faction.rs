use std::hash::Hash;

use bevy::prelude::*;

#[derive(Component, Debug, PartialEq)]
pub(crate) struct SystemFaction {
    pub system: Entity,
    pub faction: Entity,
    pub influence: f32,
    pub state: Option<String>,
}

impl Eq for SystemFaction {}

impl Hash for SystemFaction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.system.hash(state);
        self.faction.hash(state);
    }
}
