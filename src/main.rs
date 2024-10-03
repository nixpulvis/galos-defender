use std::collections::HashSet;
use bevy::prelude::*;

mod faction;
use self::faction::*;

mod system;
use self::system::{*, System};

mod expansion;
use self::expansion::*;

#[derive(Component, Debug)]
struct Name(String);

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
        .spawn((FactionBundle {
            faction: Faction,
            name: Name("Our Faction".into()),
        },))
        .id();

    let system_a = commands
        .spawn((
            SystemBundle {
                system: System,
                address: Address(0),
                name: Name("SOL".into()),
                position: Position(Vec3::splat(0.)),
            },
            Factions(HashSet::from([faction_a])),
        ))
        .id();

    let neighbor = commands
        .spawn((SystemBundle {
            system: System,
            address: Address(1),
            name: Name("ALPHA CENTAURI".into()),
            position: Position(Vec3::new(3.03125, -0.09375, 3.15625)),
        },))
        .id();

    let not_neighbor = commands
        .spawn((SystemBundle {
            system: System,
            address: Address(2),
            name: Name("G 139-21".into()),
            position: Position(Vec3::new(-17.03125, 16.875, 34.625)),
        },))
        .id();

    commands.get_entity(faction_a).map(|mut cmds| {
        cmds.insert(Systems(HashSet::from([system_a])));
    });
}

fn query(
    systems: Query<(&Name, Option<&Factions>), With<System>>,
    factions: Query<&Name, With<Faction>>,
) {
    for (system_name, system_factions) in &systems {
        dbg!(system_name);
        if let Some(sf) = system_factions {
            for faction in &sf.0 {
                let Ok(faction_name) = factions.get(*faction) else {
                    continue;
                };
                dbg!(faction_name);
            }
        }
    }
}
