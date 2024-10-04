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
            influence: EXPANSION_INFLUENCE,
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

#[cfg(test)]
mod tests {
    use crate::SystemBundle;

    use super::*;

    #[test]
    fn successful_expansion() {
        // Setup app
        let mut app = App::new();

        app.add_event::<Expand>();

        app.add_systems(Update, (check_expansion, expand).chain());

        // Setup test entities
        let system_1 = app
            .world_mut()
            .spawn(SystemBundle {
                system: System {
                    address: 0,
                    name: "System1".into(),
                },
                position: Position(Vec3::new(0., 0., 0.)),
            })
            .id();
        let our_faction = app
            .world_mut()
            .spawn(Faction {
                name: "Our Faction".into(),
            })
            .id();

        app.world_mut().spawn(SystemFaction {
            system: system_1,
            faction: our_faction,
            influence: 1.,
            state: None,
        });

        app.world_mut().spawn(SystemBundle {
            system: System {
                address: 0,
                name: "System2".into(),
            },
            position: Position(Vec3::new(1., 1., 1.)),
        });

        // Run systems
        app.update();
        app.update();

        // Check resulting changes
        let system_factions = app
            .world_mut()
            .query::<&SystemFaction>()
            .iter(app.world())
            .collect::<Vec<&SystemFaction>>();
        assert_eq!(system_factions.len(), 2);
    }

    #[test]
    fn no_expansion_low_influence() {
        // Setup app
        let mut app = App::new();

        app.add_event::<Expand>();

        app.add_systems(Update, (check_expansion, expand).chain());

        // Setup test entities
        let system_1 = app
            .world_mut()
            .spawn(SystemBundle {
                system: System {
                    address: 0,
                    name: "System1".into(),
                },
                position: Position(Vec3::new(0., 0., 0.)),
            })
            .id();
        let our_faction = app
            .world_mut()
            .spawn(Faction {
                name: "Our Faction".into(),
            })
            .id();

        app.world_mut().spawn(SystemFaction {
            system: system_1,
            faction: our_faction,
            influence: 0.,
            state: None,
        });

        app.world_mut().spawn(SystemBundle {
            system: System {
                address: 0,
                name: "System2".into(),
            },
            position: Position(Vec3::new(1., 1., 1.)),
        });

        // Run systems
        app.update();
        app.update();

        // Check resulting changes
        let system_factions = app
            .world_mut()
            .query::<&SystemFaction>()
            .iter(app.world())
            .collect::<Vec<&SystemFaction>>();
        assert_eq!(system_factions.len(), 1);
    }

    #[test]
    fn no_expansion_too_far() {
        // Setup app
        let mut app = App::new();

        app.add_event::<Expand>();

        app.add_systems(Update, (check_expansion, expand).chain());

        // Setup test entities
        let system_1 = app
            .world_mut()
            .spawn(SystemBundle {
                system: System {
                    address: 0,
                    name: "System1".into(),
                },
                position: Position(Vec3::new(0., 0., 0.)),
            })
            .id();
        let our_faction = app
            .world_mut()
            .spawn(Faction {
                name: "Our Faction".into(),
            })
            .id();

        app.world_mut().spawn(SystemFaction {
            system: system_1,
            faction: our_faction,
            influence: 1.,
            state: None,
        });

        app.world_mut().spawn(SystemBundle {
            system: System {
                address: 0,
                name: "System2".into(),
            },
            position: Position(Vec3::new(99999., 99999., 99999.)),
        });

        // Run systems
        app.update();
        app.update();

        // Check resulting changes
        let system_factions = app
            .world_mut()
            .query::<&SystemFaction>()
            .iter(app.world())
            .collect::<Vec<&SystemFaction>>();
        assert_eq!(system_factions.len(), 1);
    }
}
