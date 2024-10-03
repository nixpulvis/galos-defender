use crate::{
    faction::Faction,
    system::{Position, System},
    system_faction::SystemFaction,
    Args,
};
use bevy::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub(crate) fn spawn_data(mut commands: Commands, args: Res<Args>) {
    let data_dir = args.spawn_data.clone().expect("no arg");

    let mut faction_map = HashMap::new();
    let factions_file = File::open(data_dir.join("factions.json")).expect("open json error");
    let factions_reader = BufReader::new(factions_file);
    let factions: Value = serde_json::from_reader(factions_reader).expect("parse json error");
    for faction in factions.as_array().expect("bad json array") {
        let faction = faction.as_object().expect("bad json object");
        let id = faction
            .get("id")
            .expect("id not found")
            .as_number()
            .expect("bad id")
            .as_i64()
            .expect("bad id number") as u32;
        let name = faction
            .get("name")
            .expect("name not found")
            .as_str()
            .expect("bad name")
            .into();
        let entity = commands.spawn(Faction { name }).id();
        faction_map.insert(id, entity);
    }

    let mut system_map = HashMap::new();
    let systems_file = File::open(data_dir.join("systems.json")).expect("open json error");
    let systems_reader = BufReader::new(systems_file);
    let systems: Value = serde_json::from_reader(systems_reader).expect("parse json error");
    for system in systems.as_array().expect("bad json array") {
        let system = system.as_object().expect("bad json object");
        let address = system
            .get("address")
            .expect("address not found")
            .as_number()
            .expect("bad address")
            .as_i64()
            .expect("bad address number") as u32;
        let name = system
            .get("name")
            .expect("name not found")
            .as_str()
            .expect("bad name")
            .into();
        let position = system
            .get("position")
            .expect("position not found")
            .as_object()
            .expect("bad position")
            .get("coordinates")
            .expect("coordinates not found")
            .as_array()
            .expect("bad coordinates")
            .iter()
            .map(|c| {
                c.as_number()
                    .expect("bad coordinate")
                    .as_f64()
                    .expect("bad coordinate number") as f32
            })
            .collect::<Vec<f32>>();
        let entity = commands
            .spawn((
                System { address, name },
                Position(Vec3::new(position[0], position[1], position[2])),
            ))
            .id();
        system_map.insert(address, entity);
    }

    let system_factions_file =
        File::open(data_dir.join("system_factions.json")).expect("open json error");
    let system_factions_reader = BufReader::new(system_factions_file);
    let system_factions: Value =
        serde_json::from_reader(system_factions_reader).expect("parse json error");
    for system_faction in system_factions.as_array().expect("bad json array") {
        let system_faction = system_faction.as_object().expect("bad json object");
        let system_address = system_faction
            .get("system_address")
            .expect("system_address not found")
            .as_number()
            .expect("bad system_address")
            .as_i64()
            .expect("bad system_address number") as u32;
        let faction_id = system_faction
            .get("faction_id")
            .expect("faction_id not found")
            .as_number()
            .expect("bad faction_id")
            .as_i64()
            .expect("bad faction_id number") as u32;
        let influence = system_faction
            .get("influence")
            .expect("influence not found")
            .as_number()
            .expect("bad influence")
            .as_f64()
            .expect("bad influence number") as f32;
        let state = system_faction
            .get("state")
            .expect("state not found")
            .as_str()
            .map(|s| s.into());

        let system = system_map.get(&system_address).expect("system not found");
        let faction = faction_map.get(&faction_id).expect("faction not found");
        commands.spawn((SystemFaction {
            system: *system,
            faction: *faction,
            influence,
            state,
        },));
    }
}

pub(crate) fn spawn_manual(mut commands: Commands) {
    // Create a new faction for us while developing.
    let our_faction = commands
        .spawn((Faction {
            name: "The Power of Nates".into(),
        },))
        .id();

    let our_home = commands
        .spawn((
            System {
                address: 0,
                name: "SOL".into(),
            },
            Position(Vec3::splat(0.)),
        ))
        .id();

    // Create a presense of our faction in our home system.
    let _sf = commands
        .spawn(SystemFaction {
            system: our_home,
            faction: our_faction,
            influence: 0.79,
            state: None,
        })
        .id();

    let _neighbor = commands
        .spawn((
            System {
                address: 1,
                name: "ALPHA CENTAURI".into(),
            },
            Position(Vec3::new(3.03125, -0.09375, 3.15625)),
        ))
        .id();

    let _not_neighbor = commands
        .spawn((
            System {
                address: 2,
                name: "G 139-21".into(),
            },
            Position(Vec3::new(-17.03125, 16.875, 34.625)),
        ))
        .id();
}
