use bevy::{prelude::*, utils::dbg};
use std::collections::{hash_set, HashSet};

fn main() {
    let mut app = App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn)
        .add_systems(Update, (check_expansion, expand, query).chain())
        .add_event::<Expand>()
        .run();
}

#[derive(Component, Debug)]
struct Faction;

#[derive(Component, Debug)]
struct System;

#[derive(Component, Debug)]
struct Name(String);

#[derive(Component, Debug)]
struct Neighbors(HashSet<Entity>);

#[derive(Component, Debug)]
struct Systems(HashSet<Entity>);

#[derive(Component, Debug)]
struct Factions(HashSet<Entity>);

#[derive(Component, Debug)]
struct Address(u32);

#[derive(Component, Debug)]
struct Position(Vec3);

#[derive(Bundle, Debug)]
struct FactionBundle {
    faction: Faction,
    name: Name,
}

#[derive(Bundle, Debug)]
struct SystemBundle {
    system: System,
    address: Address,
    name: Name,
    position: Position,
}

#[derive(Event)]
struct Expand {
    faction: Entity,
    system: Entity,
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

fn expand(mut ev_r: EventReader<Expand>, mut target_system_query: Query<&mut Factions>) {
    for event in ev_r.read() {
        println!("Processing Expand event");
        // TODO: Handle creating a new Factions when it doesn't exist.
        let Ok(mut faction_list) = target_system_query.get_mut(event.system) else {
            continue;
        };
        faction_list.0.insert(event.faction);
    }
}

fn check_expansion(
    mut ev_w: EventWriter<Expand>,
    system_query: Query<(Entity, &Name, &Position, &Factions), With<System>>,
) {
    // do some additional queries on the entity lists to determine which factions should
    // expand, and where. in this example, all factions expand to all neighbors
    for (system_a, name_a, position_a, factions_a) in &system_query {
        for (system_b, name_b, position_b, factions_b) in &system_query {
            if position_a.0.distance(position_b.0) < 20. {
                for faction in factions_a.0.clone() {
                    println!("Sending Expand event");
                    ev_w.send(Expand {
                        faction,
                        system: system_b,
                    });
                }
            }
        }
    }
}
