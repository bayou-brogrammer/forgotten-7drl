use crate::{prelude::*, prompt, terrain};
use gridbugs::visible_area_detection::VisibilityGrid;

pub enum ControlFlow {
    Win,
    Upgrade,
    GetMelee,
    GameOver,
    GetRanged,
    LevelChange,
    Prompt(String),
    FireWeapon(RangedWeaponSlot),
}

impl Game {
    #[must_use]
    pub fn tick(&mut self, since_previous: Duration) -> Option<ControlFlow> {
        if self.start {
            self.start = false;
            return Some(ControlFlow::Prompt(prompt::intro()));
        }

        self.since_last_frame += since_previous;
        while let Some(remaining_since_last_frame) =
            self.since_last_frame.checked_sub(crate::world::ANIMATION_FRAME_DURATION)
        {
            self.since_last_frame = remaining_since_last_frame;
            if let Some(game_control_flow) = self.handle_tick_inner(since_previous) {
                return Some(game_control_flow);
            }
        }

        None
    }

    pub fn handle_tick_inner(&mut self, since_previous: Duration) -> Option<ControlFlow> {
        if self.world.is_gameplay_blocked() {
            if let Some((e, timer)) = self.world.components.blocks_gameplay.iter_mut().next() {
                if timer.as_millis() == 0 {
                    self.world.components.blocks_gameplay.remove(e);
                } else {
                    *timer = timer
                        .checked_sub(since_previous)
                        .map_or_else(|| Duration::from_millis(0), |remaining| remaining)
                }
            }
        }

        if let Some(countdown) = self.win_countdown {
            if countdown.as_millis() == 0 {
                return Some(ControlFlow::Win);
            } else {
                self.win_countdown = Some(
                    countdown
                        .checked_sub(since_previous)
                        .map_or_else(|| Duration::from_millis(0), |remaining| remaining),
                )
            }
        }

        self.run_systems();

        if self.is_won() {
            self.win_countdown = Some(Duration::from_secs(2));
            None
        } else if self.is_game_over() && self.win_countdown.is_none() {
            Some(ControlFlow::GameOver)
        } else {
            None
        }
    }

    pub fn generate_level(&mut self) {
        crate::log::append_entry(Message::Descend);

        let mut player_data = self.world.clone_entity_data(self.player_entity);
        for weapon in player_data.player.as_mut().unwrap().ranged_weapons.iter_mut() {
            if let Some(weapon) = weapon.as_mut() {
                if let Some(ammo) = weapon.ammo.as_mut() {
                    ammo.current = ammo.max;
                }
            }
        }

        if let Some(ammo) = player_data.player.as_mut().unwrap().melee_weapon.ammo.as_mut() {
            ammo.current = ammo.max;
        }

        let Terrain { world, agents, player_entity } =
            terrain::build_station(&mut self.terrain_state, self.world.level + 1, Some(player_data));

        self.visibility_grid = VisibilityGrid::new(world.size());
        self.behavior_context = BehaviourContext::new(world.size());

        self.world = world;
        self.agents = agents;
        self.player_entity = player_entity;

        self.prime_npcs();
        self.update_visibility();
        self.set_new_music();
    }
}
