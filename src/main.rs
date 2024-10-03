use bevy::prelude::*;
use std::collections::HashSet;

mod faction;
use faction::*;

mod system;
use system::{System, *};

mod expansion;
use expansion::*;

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn)
        .add_systems(
            Update,
            (expansion::check_expansion, expansion::expand, query).chain(),
        )
        .add_event::<Expand>()
        .run();
}

fn spawn(mut commands: Commands) {
    let faction_a = commands
        .spawn((Faction {
            name: "Our Faction".into(),
        },))
        .id();

    let system_a = commands
        .spawn((
            System {
                address: 0,
                name: "SOL".into(),
            },
            Position(Vec3::splat(0.)),
            Factions(HashSet::from([faction_a])),
        ))
        .id();

    let neighbor = commands
        .spawn((
            System {
                address: 1,
                name: "ALPHA CENTAURI".into(),
            },
            Position(Vec3::new(3.03125, -0.09375, 3.15625)),
        ))
        .id();

    let not_neighbor = commands
        .spawn((
            System {
                address: 2,
                name: "G 139-21".into(),
            },
            Position(Vec3::new(-17.03125, 16.875, 34.625)),
        ))
        .id();

    commands.get_entity(faction_a).map(|mut cmds| {
        cmds.insert(Systems(HashSet::from([system_a])));
    });
}

fn query(systems: Query<(&System, Option<&Factions>)>, factions: Query<&Faction>) {
    for (system, system_factions) in &systems {
        dbg!(system);
        if let Some(sf) = system_factions {
            for faction_ent in &sf.0 {
                let Ok(faction) = factions.get(*faction_ent) else {
                    error!("Entity in faction list does not exist");
                    continue;
                };
                dbg!(faction);
            }
        }
    }
}
