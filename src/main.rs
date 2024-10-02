use bevy::{prelude::*, utils::dbg};

fn main() {
    let mut app = App::new()
        // .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn)
        .add_systems(Update, query)
        .add_systems(Update, check_expansion)
        .add_systems(Update, expand)
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
struct Neighbors(Vec<Entity>);

#[derive(Component, Debug)]
struct Systems(Vec<Entity>);

#[derive(Component, Debug)]
struct Factions(Vec<Entity>);

#[derive(Component, Debug)]
struct Address(u32);

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
            },
            Factions(vec![faction_a]),
        ))
        .id();

    let system_b = commands
        .spawn((
            SystemBundle {
                system: System,
                address: Address(1),
                name: Name("Neighbor 1".into()),
            },
            Neighbors(vec![system_a]),
        ))
        .id();

    commands.get_entity(system_a).map(|mut cmds| {
        cmds.insert(Neighbors(vec![system_b]));
    });
    commands.get_entity(faction_a).map(|mut cmds| {
        cmds.insert(Systems(vec![system_a]));
    });
}

fn query(systems: Query<(&Name, &Factions), With<System>>, factions: Query<&Name, With<Faction>>) {
    for (system_name, system_factions) in &systems {
        dbg!(system_name);
        for faction in &system_factions.0 {
            let Ok(faction_name) = factions.get(*faction) else {
                continue;
            };
            dbg!(faction_name);
        }
    }
}

fn expand(mut ev_r: EventReader<Expand>, mut target_system_query: Query<&mut Factions>) {
    for event in ev_r.read() {
        let Ok(mut faction_list) = target_system_query.get_mut(event.system) else {
            continue;
        };
        faction_list.0.push(event.faction);
    }
}

fn check_expansion(
    mut ev_w: EventWriter<Expand>,
    system_query: Query<(&Factions, &Neighbors), With<System>>,
) {
    for (factions, neighbors) in &system_query {
        // do some additional queries on the entity lists to determine which factions should
        // expand, and where. in this example, all factions expand to all neighbors
        for neighbor in neighbors.0.clone() {
            for faction in factions.0.clone() {
                ev_w.send(Expand {
                    faction,
                    system: neighbor,
                });
            }
        }
    }
}
