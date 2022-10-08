use crate::{Agent, World};
use gridbugs::{entity_table::ComponentTable, spatial_table::Coord};

pub struct EnemyCounts {
    orc: Vec<usize>,
    troll: Vec<usize>,
}

impl EnemyCounts {
    fn new() -> Self {
        Self { orc: vec![8, 10, 10, 12, 12], troll: vec![2, 2, 4, 6, 6] }
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

    for _ in 0..enemy_count.orc[index] {
        if let Some(coord) = npc_candidates.pop() {
            println!("Spawning orc at {:?}", coord);
            let orc = world.spawn_orc(coord);
            agents.insert(orc, Agent::new(world.size()));
        }
    }
    for _ in 0..enemy_count.troll[index] {
        if let Some(coord) = npc_candidates.pop() {
            println!("Spawning troll at {:?}", coord);
            let troll = world.spawn_troll(coord);
            agents.insert(troll, Agent::new(world.size()));
        }
    }
}
