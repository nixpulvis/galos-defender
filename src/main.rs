use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, spawn);
    app.add_systems(Update, query);
    app.run();
}

#[derive(Component, Debug)]
struct Faction {
    name: String,
}

#[derive(Component, Debug)]
struct System {
    address: u32,
    name: String,
}

#[derive(Component, Debug)]
struct WithEntities {
    entities: Vec<Entity>,
}

fn spawn(mut commands: Commands) {
    let faction_a = commands
        .spawn((Faction {
            name: "Our Faction".into(),
        },))
        .id();
    let system_a = commands
        .spawn((System {
            address: 0,
            name: "SOL".into(),
        },))
        .id();

    commands.get_entity(system_a).map(|mut cmds| {
        cmds.insert(WithEntities {
            entities: vec![faction_a],
        });
    });
    commands.get_entity(faction_a).map(|mut cmds| {
        cmds.insert(WithEntities {
            entities: vec![system_a],
        });
    });
}

fn query(systems: Query<(&System, &WithEntities)>, factions: Query<&Faction>) {
    for (system, system_entites) in &systems {
        for faction_entity in &system_entites.entities {
            let faction = factions.get(*faction_entity);
            dbg!(system, faction);
        }
    }
}
