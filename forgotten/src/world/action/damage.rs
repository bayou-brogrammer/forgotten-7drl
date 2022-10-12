use crate::{
    prelude::*,
    world::{explosion, realtime},
};

const KNOCKBACK: usize = 3;

impl World {
    pub fn melee_attack(&mut self, attacker: Entity, victim: Entity, direction: CardinalDirection) {
        if self.components.player.get(attacker).is_some() {
            self.player_melee_attack(attacker, victim, direction);
        } else if self.components.player.get(victim).is_some() {
            self.npc_melee_attack(attacker, victim);
        }
    }

    pub fn player_melee_attack(&mut self, attacker: Entity, victim: Entity, direction: CardinalDirection) {
        let player = self.components.player.get_mut(attacker).unwrap();
        let remove = if let Some(ammo) = player.melee_weapon.ammo.as_mut() {
            ammo.current = ammo.current.saturating_sub(1);
            ammo.current == 0
        } else {
            false
        };

        crate::event::add_event(ExternalEvent::SoundEffect(match player.melee_weapon.name {
            WeaponType::CattleProd => SoundEffect::CattleProd,
            WeaponType::Chainsaw => SoundEffect::Chainsaw,
            _ => SoundEffect::Punch,
        }));

        if let Some(enemy) = self.components.npc.get(victim) {
            crate::log::append_entry(Message::PlayerHitEnemy {
                enemy: enemy.npc_type,
                weapon: player.melee_weapon.name,
            });
        }

        let pen = player.melee_pen();
        if pen >= self.components.armour.get(victim).expect("npc lacks armour").value {
            let mut dmg = player.melee_dmg();
            if player.traits.double_damage {
                dmg *= 2;
            }
            self.damage_character(victim, dmg);
        }

        let player = self.components.player.get(attacker).unwrap();
        let stun = player.stun_percent();
        for ability in player.melee_weapon.abilities.clone() {
            use WeaponAbility::*;
            match ability {
                KnockBack => {
                    self.components.pushed_from.insert(victim, self.spatial_table.coord_of(victim).unwrap());
                    self.components.realtime.insert(victim, ());
                    self.realtime_components.movement.insert(
                        victim,
                        realtime::movement::spec::Movement {
                            path: direction.coord(),
                            repeat: realtime::movement::spec::Repeat::Steps(KNOCKBACK),
                            cardinal_step_duration: Duration::from_millis(50),
                        }
                        .build(),
                    );
                }
                Shock => {
                    if self.apply_stun(victim, stun) {
                        if let Some(npc) = self.components.npc.get(victim) {
                            crate::log::append_entry(Message::EnemyStunend(npc.npc_type));
                        }
                    }

                    self.spawn_flash(
                        self.spatial_table.coord_of(victim).unwrap(),
                        Some(Rgb24 { r: 255, g: 255, b: 0 }),
                    );
                }
                _ => (),
            }
        }

        let player = self.components.player.get_mut(attacker).unwrap();
        if remove {
            player.melee_weapon = Weapon::new_bare_hands();
        }
    }

    pub fn npc_melee_attack(&mut self, attacker: Entity, victim: Entity) {
        let &damage = self.components.damage.get(attacker).expect("npc lacks damage component");
        let npc_type = self.components.npc.get(attacker).expect("npc lacks npc component").npc_type;

        let stun_percentage = match npc_type {
            NpcType::MiniBot => 10,
            NpcType::SecBot => 20,
            NpcType::RoboCop => 25,
            NpcType::DoomBot => 40,
        };

        if self.apply_stun(victim, stun_percentage) {
            crate::log::append_entry(Message::PlayerStunned);
        }

        crate::log::append_entry(Message::EnemyHitPlayer(npc_type));
        self.damage_character(victim, damage);
    }

    pub fn damage_character(&mut self, character: Entity, hit_points_to_lose: u32) {
        if self.components.dead.contains(character) {
            // prevent cascading damage on explosions
            return;
        }

        let hit_points = self.components.hp.get_mut(character).expect("character lacks hit_points");
        if hit_points_to_lose >= hit_points.current {
            hit_points.current = 0;
            self.character_die(character);
        } else {
            hit_points.current -= hit_points_to_lose;
        }
    }

    fn character_die(&mut self, character: Entity) {
        if self.components.player.contains(character) {
            crate::event::add_event(ExternalEvent::SoundEffect(SoundEffect::Die));
            crate::log::append_entry(Message::PlayerDies);
        } else if let Some(enemy) = self.components.npc.get(character) {
            crate::log::append_entry(Message::EnemyDies(enemy.npc_type));
        }

        self.components.dead.insert(character, ());

        if self.components.explodes_on_death.contains(character) {
            if let Some(coord) = self.spatial_table.coord_of(character) {
                self.components.explodes_on_death.remove(character);

                use explosion::spec::*;
                let spec = Explosion {
                    mechanics: Mechanics(2),
                    particle_emitter: ParticleEmitter {
                        duration: Duration::from_millis(400),
                        num_particles_per_frame: 100,
                        min_step: Duration::from_millis(100),
                        max_step: Duration::from_millis(300),
                        fade_duration: Duration::from_millis(500),
                    },
                };
                crate::log::append_entry(Message::DoomBotExplodes);
                explosion::explode(self, coord, spec);
            }
        }
    }
}

// Projectiles
impl World {
    pub fn apply_projectile_damage(
        &mut self,
        projectile_entity: Entity,
        mut projectile_damage: ProjectileDamage,
        projectile_movement_direction: Direction,
        entity_to_damage: Entity,
    ) {
        if let Some(armour) = self.components.armour.get(entity_to_damage).cloned() {
            if let Some(remaining_pen) = projectile_damage.pen.checked_sub(armour.value) {
                if let Some(enemy) = self.components.npc.get(entity_to_damage) {
                    if let Some(weapon) = projectile_damage.weapon_name {
                        crate::log::append_entry(Message::PlayerHitEnemy { enemy: enemy.npc_type, weapon })
                    }
                }

                let damage = projectile_damage.hit_points;
                let victim_health =
                    self.components.hp.get(entity_to_damage).map(|hp| hp.current).unwrap_or(0);
                let actual_damage = damage.min(victim_health);
                self.damage_character(entity_to_damage, damage);

                // Get some health back
                if projectile_damage.life_steal {
                    if let Some(player) = self.components.player.entities().next() {
                        if let Some(hit_points) = self.components.hp.get_mut(player) {
                            hit_points.current = (hit_points.current + actual_damage).min(hit_points.max);
                        }
                    }
                }
                // Push them back!
                if projectile_damage.push_back {
                    self.components.realtime.insert(entity_to_damage, ());
                    self.realtime_components.movement.insert(
                        entity_to_damage,
                        realtime::movement::spec::Movement {
                            path: projectile_movement_direction.coord(),
                            repeat: realtime::movement::spec::Repeat::Steps(KNOCKBACK),
                            cardinal_step_duration: Duration::from_millis(100),
                        }
                        .build(),
                    );
                }

                if remaining_pen > 0 {
                    projectile_damage.pen = remaining_pen;
                    self.components.projectile_damage.insert(projectile_entity, projectile_damage);
                } else {
                    self.components.remove_entity(projectile_entity);
                }
            } else {
                self.components.remove_entity(projectile_entity);
            }
        }
    }

    pub fn heal_fully(&mut self, entity: Entity) {
        if let Some(hit_points) = self.components.hp.get_mut(entity) {
            crate::event::add_event(ExternalEvent::SoundEffect(SoundEffect::Heal));
            crate::log::append_entry(Message::Heal);
            hit_points.current = hit_points.max;
        }
    }
}
