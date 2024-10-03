use bevy::prelude::*;

mod faction;
use faction::*;

mod system;
use system::{System, *};

mod system_faction;
use system_faction::SystemFaction;

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
            influence: 79.,
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
        println!(
            "{} in {} with {} influence.",
            &faction.name, &system.name, system_faction.influence
        );
    }
}
// fn query(systems: Query<(&System, Option<&Factions>)>, factions: Query<&Faction>) {
//     for (system, system_factions) in &systems {
//         dbg!(system);
//         if let Some(sf) = system_factions {
//             for faction_ent in &sf.0 {
//                 let Ok(faction) = factions.get(*faction_ent) else {
//                     error!("Entity in faction list does not exist");
//                     continue;
//                 };
//                 dbg!(faction);
//             }
//         }
//     }
// }
