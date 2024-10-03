use bevy::prelude::*;
use clap::Parser;
use std::path::PathBuf;

mod faction;
use faction::*;

mod system;
use system::{System, *};

mod system_faction;
use system_faction::SystemFaction;

mod spawn;
use spawn::*;

mod expansion;
use expansion::*;

#[derive(Parser, Resource, Debug)]
#[command(version, about)]
pub(crate) struct Args {
    spawn_data: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut app = App::new();
    // app.add_plugins(DefaultPlugins);

    if args.spawn_data.is_some() {
        app.add_systems(Startup, spawn_data);
    } else {
        app.add_systems(Startup, spawn_manual);
    }
    app.insert_resource(args);

    app.add_plugins(expansion::plugin);

    app.add_systems(Update, query.after(Expansion));

    app.run();
}

fn query(
    systems: Query<&System>,
    factions: Query<&Faction>,
    system_factions: Query<&SystemFaction>,
) {
    for system_faction in &system_factions {
        let system = systems.get(system_faction.system).expect("missing system");
        let faction = factions
            .get(system_faction.faction)
            .expect("missing faction");
        print!(
            "{} @ {}", &faction.name, &system.name);
        if let Some(state) = &system_faction.state {
            print!(" in state {}", state);
        }
        println!(" with {} influence.", system_faction.influence);
    }
}
