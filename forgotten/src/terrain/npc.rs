use crate::{Agent, NpcType, World};
use gridbugs::{entity_table::ComponentTable, spatial_table::Coord};

pub struct EnemyCounts {
    mini: Vec<usize>,
    sec: Vec<usize>,
    sentry: Vec<usize>,
    doom: Vec<usize>,
}

impl EnemyCounts {
    fn new() -> Self {
        Self {
            mini: vec![8, 10, 10, 12, 12],
            sec: vec![2, 2, 4, 6, 6],
            sentry: vec![2, 3, 3, 4, 4],
            doom: vec![0, 0, 1, 2, 4],
        }
    }
}

pub fn generate_npcs(
    world: &mut World,
    level: u32,
    npc_candidates: &mut Vec<Coord>,
    agents: &mut ComponentTable<Agent>,
) {
    crate::rng::shuffle(npc_candidates);

    let index = level as usize - 1;
    let enemy_count = EnemyCounts::new();

    for _ in 0..enemy_count.mini[index] {
        if let Some(coord) = npc_candidates.pop() {
            let mini = world.spawn_minibot(coord);
            agents.insert(mini, Agent::new(world.size(), NpcType::MiniBot));
        }
    }
    for _ in 0..enemy_count.sec[index] {
        if let Some(coord) = npc_candidates.pop() {
            let sec_bot = world.spawn_secbot(coord);
            agents.insert(sec_bot, Agent::new(world.size(), NpcType::SecBot));
        }
    }
    for _ in 0..enemy_count.sentry[index] {
        if let Some(coord) = npc_candidates.pop() {
            println!("Spawning sentry at {:?}", coord);
            // let sentry = world.spawn_sentry(coord);
            // agents.insert(sentry, Agent::new(world.size()));
        }
    }
    for _ in 0..enemy_count.doom[index] {
        if let Some(coord) = npc_candidates.pop() {
            println!("Spawning doom at {:?}", coord);
            // let doom = world.spawn_doom(coord);
            // agents.insert(doom, Agent::new(world.size()));
        }
    }
}
