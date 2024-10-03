use bevy::prelude::*;

#[derive(Component, Debug)]
pub(crate) struct SystemFaction {
    pub system: Entity,
    pub faction: Entity,
    pub influence: f32,
    pub state: Option<String>,
}
