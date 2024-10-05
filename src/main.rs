use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
};
use clap::Parser;
use std::path::PathBuf;

mod faction;
use faction::*;

mod system;
use system::System;

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
    app.add_plugins(DefaultPlugins.set(RenderPlugin {
        render_creation: bevy::render::settings::RenderCreation::Automatic(WgpuSettings {
            backends: Some(Backends::VULKAN),
            ..Default::default()
        }),
        ..Default::default()
    }));

    if args.spawn_data.is_some() {
        app.add_systems(Startup, spawn_data);
    } else {
        app.add_systems(Startup, spawn_manual);
    }
    app.insert_resource(args);
    app.insert_resource(InfluenceTimer(Timer::from_seconds(
        1.0,
        TimerMode::Repeating,
    )));

    app.add_plugins(expansion::plugin);

    app.add_systems(Update, tick.before(Expansion));
    // app.add_systems(Update, query.after(Expansion));

    app.run();
}

#[derive(Resource)]
struct InfluenceTimer(Timer);

fn tick(
    systems: Query<&System>,
    factions: Query<&Faction>,
    mut system_factions: Query<&mut SystemFaction>,
    mut timer: ResMut<InfluenceTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        for mut system_faction in &mut system_factions {
            let system = systems.get(system_faction.system).expect("no system");
            let faction = factions.get(system_faction.faction).expect("no faction");
            system_faction.influence += 0.025;
            println!(
                "ticking {} @ {} to {}",
                faction.name, system.name, system_faction.influence
            );
        }
    }
}

fn _query(
    systems: Query<&System>,
    factions: Query<&Faction>,
    system_factions: Query<&SystemFaction>,
) {
    for system_faction in &system_factions {
        let system = systems.get(system_faction.system).expect("missing system");
        let faction = factions
            .get(system_faction.faction)
            .expect("missing faction");
        print!("{} @ {}", &faction.name, &system.name);
        if let Some(state) = &system_faction.state {
            print!(" in state {}", state);
        }
        println!(" with {} influence.", system_faction.influence);
    }
}
