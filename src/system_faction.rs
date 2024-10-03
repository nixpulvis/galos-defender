use bevy::prelude::*;

// TODO: OPT: impl Hash for faster comparisons than .contains() in expand system

#[derive(Component, Debug)]
pub(crate) struct SystemFaction {
    pub system: Entity,
    pub faction: Entity,
    pub influence: f32,
    pub state: Option<String>,
}

impl PartialEq for SystemFaction {
    fn eq(&self, other: &Self) -> bool {
        (self.system == other.system) && (self.faction == other.faction)
    }
}
