use crate::{
    system::{Position, System},
    system_faction::SystemFaction,
    Faction,
};
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Event)]
pub(crate) struct Expand {
    faction: Entity,
    system: Entity,
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
    system_factions: Query<&SystemFaction>,
    mut commands: Commands,
) {
    for event in ev_r.read() {
        info!("Processing Expand event");
        let Ok(_system) = systems.get(event.system) else {
            error!("System in expansion event does not exist");
            continue;
        };

        let Ok(_faction) = factions.get(event.faction) else {
            error!("Faction in expansion event does not exist");
            continue;
        };

        let new_sys_faction = SystemFaction {
            system: event.system,
            faction: event.faction,
            influence: 25.,
            state: None,
        };
        let all_system_factions = system_factions.iter().collect::<HashSet<&SystemFaction>>();
        if all_system_factions.contains(&&new_sys_faction) {
            return;
        }

        commands.spawn(new_sys_faction);
    }
}

pub(crate) fn check_expansion(
    mut ev_w: EventWriter<Expand>,
    systems: Query<(Entity, &System, &Position)>,
    factions: Query<&Faction>,
    system_factions: Query<&SystemFaction>,
) {
    for system_faction in &system_factions {
        if system_faction.influence >= 0.75 {
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
                        system: dst_system_id,
                        faction: system_faction.faction,
                    });

                    // TODO: Only expand to the closest system.
                    break;
                }
            }
        }
    }
}
