use crate::prelude::*;
use gridbugs::{
    grid_2d::{Coord, Size},
    grid_search_cardinal::{
        best::{BestSearch, Context as BestSearchContext, Depth},
        distance_map::{
            Distance, DistanceMap, PopulateContext as DistanceMapPopulateContext,
            SearchContext as DistanceMapSearchContext,
        },
        point_to_point::{expand, Context as PointToPointSearchContext, NoPath},
        CanEnter, Path, Step,
    },
    line_2d::LineSegment,
    visible_area_detection::{vision_distance, VisibilityGrid, VisionDistance},
};

struct WorldCanEnterIgnoreCharacters<'a> {
    world: &'a World,
}

impl<'a> CanEnter for WorldCanEnterIgnoreCharacters<'a> {
    fn can_enter(&self, coord: Coord) -> bool {
        self.world.can_npc_traverse_feature_at_coord(coord)
    }
}

struct WorldCanEnterAvoidNpcs<'a> {
    world: &'a World,
}

impl<'a> CanEnter for WorldCanEnterAvoidNpcs<'a> {
    fn can_enter(&self, coord: Coord) -> bool {
        self.world.can_npc_traverse_feature_at_coord(coord) && !self.world.is_npc_at_coord(coord)
    }

    fn can_step(&self, step: Step) -> bool {
        self.can_enter(step.to_coord)
    }
}

const MAX_DISTANCE: Distance = 5;

fn has_line_of_sight(
    eye: Coord,
    dest: Coord,
    world: &World,
    vision_distance: vision_distance::Circle,
) -> bool {
    for coord in LineSegment::new(eye, dest).iter() {
        let eye_to_coord = coord - eye;
        if !vision_distance.in_range(eye_to_coord) {
            return false;
        }
        if !world.can_npc_see_through_feature_at_coord(coord) {
            return false;
        }
    }
    true
}

#[derive(Serialize, Deserialize)]
pub struct BehaviourContext {
    best_search_context: BestSearchContext,
    point_to_point_search_context: PointToPointSearchContext,
    distance_map_populate_context: DistanceMapPopulateContext,
    distance_map_search_context: DistanceMapSearchContext,
    player_approach: DistanceMap,
    player_flee: DistanceMap,
    wander_path: Path,
}

impl BehaviourContext {
    pub fn new(size: Size) -> Self {
        Self {
            best_search_context: BestSearchContext::new(size),
            point_to_point_search_context: PointToPointSearchContext::new(size),
            distance_map_populate_context: DistanceMapPopulateContext::default(),
            distance_map_search_context: DistanceMapSearchContext::new(size),
            player_approach: DistanceMap::new(size),
            player_flee: DistanceMap::new(size),
            wander_path: Path::default(),
        }
    }

    pub fn update(&mut self, player: Entity, world: &World) {
        if let Some(player_coord) = world.entity_coord(player) {
            let can_enter = WorldCanEnterIgnoreCharacters { world };
            self.distance_map_populate_context.add(player_coord);
            self.distance_map_populate_context.populate_approach(&can_enter, 20, &mut self.player_approach);
            self.distance_map_populate_context.add(player_coord);
            self.distance_map_populate_context.populate_flee(&can_enter, 20, &mut self.player_flee);
        } else {
            self.player_approach.clear();
            self.player_flee.clear();
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum Behaviour {
    Flee,
    Wander { avoid: bool },
    Chase { last_seen_player_coord: Coord, accurate: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcAction {
    Walk(CardinalDirection),
    Wait,
    FireLaser(CardinalDirection),
}

struct Wander<'a> {
    world: &'a World,
    last_seen_grid: &'a VisibilityGrid<LastSeenCell>,
    min_last_seen_coord: Option<Coord>,
    min_last_seen_count: u64,
    entity: Entity,
    avoid: bool,
}

impl<'a> BestSearch for Wander<'a> {
    fn is_at_max_depth(&self, _depth: Depth) -> bool {
        false
    }

    fn can_enter_initial_updating_best(&mut self, coord: Coord) -> bool {
        if self.world.can_npc_traverse_feature_at_coord(coord) {
            if let Some(entity) = self.world.get_character_at_coord(coord) {
                if entity != self.entity {
                    let my_coord = self.world.entity_coord(self.entity).unwrap();
                    if my_coord.manhattan_distance(coord) < 4 {
                        let can_see_character = has_line_of_sight(
                            my_coord,
                            coord,
                            self.world,
                            vision_distance::Circle::new_squared(40),
                        );

                        if can_see_character && crate::rng::range(0u8..4) > 0 {
                            return false;
                        }
                    }
                }
            }

            if let Some(last_seen_cell) = self.last_seen_grid.get_data(coord) {
                if self.avoid && last_seen_cell.avoid_until > self.min_last_seen_count {
                    return false;
                }

                let last_seen_count = last_seen_cell.count;
                if last_seen_count < self.min_last_seen_count {
                    self.min_last_seen_count = last_seen_count;
                    self.min_last_seen_coord = Some(coord);
                }

                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn can_step_updating_best(&mut self, step: Step) -> bool {
        self.can_enter_initial_updating_best(step.to_coord)
    }

    fn best_coord(&self) -> Option<Coord> {
        self.min_last_seen_coord
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    npc_type: NpcType,
    behaviour: Behaviour,
    vision_distance: vision_distance::Circle,
    last_seen_grid: VisibilityGrid<LastSeenCell>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
struct LastSeenCell {
    count: u64,
    avoid_until: u64,
}

impl LastSeenCell {
    pub fn update(&mut self, coord: Coord, can_see_player: bool, behaviour_context: &BehaviourContext) {
        self.count += 1;
        let distance_map_to_player = &behaviour_context.player_approach;

        if can_see_player {
            if let Some(distance_to_player) = distance_map_to_player.distance(coord) {
                if distance_to_player < MAX_DISTANCE {
                    self.avoid_until = self.count + 20;
                }
            }
        }
    }
}

impl Agent {
    pub fn new(size: Size, npc_type: NpcType) -> Self {
        Self {
            npc_type,
            last_seen_grid: VisibilityGrid::new(size),
            vision_distance: vision_distance::Circle::new_squared(40),
            behaviour: Behaviour::Wander { avoid: true },
        }
    }

    pub fn act(
        &mut self,
        entity: Entity,
        world: &World,
        player: Entity,
        behaviour_context: &mut BehaviourContext,
    ) -> NpcAction {
        let coord = if let Some(coord) = world.entity_coord(entity) {
            coord
        } else {
            return NpcAction::Wait;
        };

        let npc = world.entity_npc(entity);

        self.behaviour = if let Some(player_coord) = world.entity_coord(player) {
            let can_see_player = has_line_of_sight(coord, player_coord, world, self.vision_distance);
            self.last_seen_grid.update_custom(AMBIENT_COL, world, self.vision_distance, coord, |d, c| {
                d.update(c, can_see_player, behaviour_context)
            });

            if can_see_player {
                match npc.disposition {
                    Disposition::Hostile => {
                        Behaviour::Chase { last_seen_player_coord: player_coord, accurate: true }
                    }
                    Disposition::Afraid => {
                        if behaviour_context.player_approach.distance(coord).unwrap() < MAX_DISTANCE {
                            Behaviour::Flee
                        } else {
                            Behaviour::Wander { avoid: true }
                        }
                    }
                }
            } else {
                match self.behaviour {
                    Behaviour::Chase { last_seen_player_coord, .. } => {
                        if last_seen_player_coord == coord {
                            // walk up to where the player was last seen, then go back to wandering
                            let avoid = npc.disposition == Disposition::Afraid;
                            Behaviour::Wander { avoid }
                        } else {
                            Behaviour::Chase {
                                last_seen_player_coord,
                                accurate: last_seen_player_coord == coord,
                            }
                        }
                    }
                    Behaviour::Wander { avoid } => Behaviour::Wander { avoid },
                    Behaviour::Flee => {
                        // stop fleeing the player if you can't see them
                        Behaviour::Wander { avoid: true }
                    }
                }
            }
        } else {
            Behaviour::Wander { avoid: false }
        };

        match self.behaviour {
            Behaviour::Flee => NpcAction::Wait,
            Behaviour::Wander { avoid } => {
                let mut path_node = behaviour_context.wander_path.pop();
                let need_new_path = if let Some(path_node) = path_node {
                    let implied_current_coord = path_node.to_coord - path_node.in_direction.coord();
                    implied_current_coord != coord
                } else {
                    true
                };

                if need_new_path {
                    // let (coord, _) = self
                    //     .last_seen_grid
                    //     .enumerate()
                    //     .filter(|(_, c)| *c != CellVisibility::Never)
                    //     .choose(&mut rand::thread_rng())
                    //     .unwrap();

                    behaviour_context.best_search_context.best_search_path(
                        Wander {
                            world,
                            last_seen_grid: &self.last_seen_grid,
                            min_last_seen_coord: None,
                            min_last_seen_count: self.last_seen_grid.get_data(coord).unwrap().count,
                            entity,
                            avoid,
                        },
                        coord,
                        &mut behaviour_context.wander_path,
                    );

                    path_node = behaviour_context.wander_path.pop();
                }

                if let Some(path_node) = path_node {
                    NpcAction::Walk(path_node.in_direction)
                } else {
                    NpcAction::Wait
                }
            }
            Behaviour::Chase { last_seen_player_coord, accurate } => {
                if accurate {
                    // if self.npc_type == NpcType::MiniBot {
                    //     let line = LineSegment::new(coord, last_seen_player_coord);
                    //     if line.num_steps() == 2 && line.num_steps() == line.num_cardinal_steps() {
                    //         let direction = match last_seen_player_coord.x.cmp(&coord.x) {
                    //             Ordering::Equal => match last_seen_player_coord.y.cmp(&coord.y) {
                    //                 Ordering::Equal => unreachable!(),
                    //                 Ordering::Less => CardinalDirection::North,
                    //                 Ordering::Greater => CardinalDirection::South,
                    //             },
                    //             Ordering::Less => CardinalDirection::West,
                    //             Ordering::Greater => CardinalDirection::East,
                    //         };

                    //         return NpcAction::FireLaser(direction);
                    //     }
                    // }

                    let maybe_cardinal_direction =
                        behaviour_context.distance_map_search_context.search_first(
                            &WorldCanEnterAvoidNpcs { world },
                            coord,
                            MAX_DISTANCE,
                            &behaviour_context.player_approach,
                        );

                    match maybe_cardinal_direction {
                        None => {
                            self.behaviour = Behaviour::Wander { avoid: true };
                            NpcAction::Wait
                        }
                        Some(cardinal_direction) => NpcAction::Walk(cardinal_direction),
                    }
                } else {
                    let result = behaviour_context.point_to_point_search_context.point_to_point_search_first(
                        expand::JumpPoint,
                        &WorldCanEnterAvoidNpcs { world },
                        coord,
                        last_seen_player_coord,
                    );

                    match result {
                        Err(NoPath) | Ok(None) => {
                            self.behaviour = Behaviour::Wander { avoid: true };
                            NpcAction::Wait
                        }
                        Ok(Some(cardinal_direction)) => NpcAction::Walk(cardinal_direction),
                    }
                }
            }
        }
    }
}
