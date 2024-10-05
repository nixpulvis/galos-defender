use crate::{
    system::{Position, System},
    system_faction::SystemFaction,
    Faction,
};
use bevy::prelude::*;
use std::collections::HashSet;

pub(crate) const EXPANSION_INFLUENCE: f32 = 0.15;
pub(crate) const EXPANSION_INFLUENCE_THRESHOLD: f32 = 0.75;

#[derive(Event)]
pub(crate) struct Expand {
    source_system_faction: Entity,
    destination_system: Entity,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Expansion;

pub(crate) fn plugin(app: &mut App) {
    app.add_event::<Expand>();
    app.add_systems(Update, (check_expansion, expand).chain().in_set(Expansion));
}

pub(crate) fn expand(
    mut ev_r: EventReader<Expand>,
    systems: Query<&System>,
    factions: Query<&Faction>,
    mut system_factions: Query<&mut SystemFaction>,
    mut commands: Commands,
) {
    for event in ev_r.read() {
        info!("Processing Expand event");
        let Ok(src) = system_factions.get(event.source_system_faction) else {
            error!("Source system faction in expansion event does not exist");
            continue;
        };

        let Ok(_src_sys) = systems.get(src.system) else {
            error!("Source system in expansion event does not exist");
            continue;
        };

        let Ok(_src_fac) = factions.get(src.faction) else {
            error!("Source faction in expansion event does not exist");
            continue;
        };

        let Ok(_dst_sys) = systems.get(event.destination_system) else {
            error!("Destination system in expansion event does not exist");
            continue;
        };

        let new_sys_faction = SystemFaction {
            system: event.destination_system,
            faction: src.faction,
            influence: EXPANSION_INFLUENCE,
            state: None,
        };

        let all_system_factions = system_factions.iter().collect::<HashSet<&SystemFaction>>();
        if all_system_factions.contains(&new_sys_faction) {
            return;
        }

        let Ok(mut src) = system_factions.get_mut(event.source_system_faction) else {
            error!("Source system faction in expansion event does not exist");
            continue;
        };
        src.influence -= EXPANSION_INFLUENCE;
        commands.spawn(new_sys_faction);
    }
}

pub(crate) fn check_expansion(
    mut ev_w: EventWriter<Expand>,
    systems: Query<(Entity, &System, &Position)>,
    factions: Query<&Faction>,
    system_factions: Query<(Entity, &SystemFaction)>,
) {
    for (entity, system_faction) in &system_factions {
        if system_faction.influence >= EXPANSION_INFLUENCE_THRESHOLD {
            let (src_system_id, src_system, src_position) =
                systems.get(system_faction.system).expect("missing system");
            let faction = factions
                .get(system_faction.faction)
                .expect("missing faction");
            info!(
                "{} in {} ready for expansion",
                faction.name, src_system.name
            );
            for (dst_system_id, dst_system, dst_position) in &systems {
                if src_system_id == dst_system_id {
                    continue;
                }

                // TODO: Check more conditions, like faction count, war state, etc.
                // TODO: Check within 20Ly cube around src.
                if src_position.0.distance(dst_position.0) < 20. {
                    info!(
                        "Sending Expand event from {} to {}",
                        src_system.name, dst_system.name
                    );
                    ev_w.send(Expand {
                        source_system_faction: entity,
                        destination_system: dst_system_id,
                    });

                    // TODO: Only expand to the closest system.
                    break;
                }
            }
        }
    }
}
