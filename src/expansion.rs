use std::collections::HashSet;
use bevy::prelude::*;
use crate::{
    Name,
    system::{System, Position, Factions},
};

#[derive(Event)]
pub(crate) struct Expand {
    faction: Entity,
    system: Entity,
}

pub(crate) fn expand(
    mut ev_r: EventReader<Expand>,
    mut target_system_query: Query<(Entity, &Name, Option<&mut Factions>)>,
    mut commands: Commands,
) {
    for event in ev_r.read() {
        println!("Processing Expand event");
        let Ok((entity, name, faction_list)) = target_system_query.get_mut(event.system) else {
            continue;
        };
        if let Some(mut fl) = faction_list {
            println!("pushing {}", &name.0);
            fl.0.insert(event.faction);
        } else {
            println!("inserting {}", &name.0);
            commands
                .entity(entity)
                .insert(Factions(HashSet::from([event.faction])));
        }
    }
}

pub(crate) fn check_expansion(
    mut ev_w: EventWriter<Expand>,
    system_query: Query<(Entity, &Name, &Position, Option<&Factions>), With<System>>,
) {
    // do some additional queries on the entity lists to determine which factions should
    // expand, and where. in this example, all factions expand to all neighbors
    for (system_a, name_a, position_a, factions_a) in &system_query {
        for (system_b, name_b, position_b, factions_b) in &system_query {
            if system_a == system_b {
                continue;
            }
            if position_a.0.distance(position_b.0) < 20. {
                if let Some(factions_a) = factions_a {
                    for faction in factions_a.0.clone() {
                        println!("Sending Expand event from {} to {}", name_a.0, name_b.0);
                        ev_w.send(Expand {
                            faction,
                            system: system_b,
                        });
                    }
                }
            }
        }
    }
}
