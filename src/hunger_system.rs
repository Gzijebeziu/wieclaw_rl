use specs::prelude::*;
use super::{HungerClock, RunState, HungerState, SufferDamage, gamelog::GameLog, MyTurn};

pub struct HungerSystem {}

impl<'a> System<'a> for HungerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
                        Entities<'a>,
                        WriteStorage<'a, HungerClock>,
                        ReadExpect<'a, Entity>,
                        ReadExpect<'a, RunState>,
                        WriteStorage<'a, SufferDamage>,
                        WriteExpect<'a, GameLog>,
                        ReadStorage<'a, MyTurn>
                    );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut hunger_clock, player_entity, _runstate, mut inflict_damage, mut log, turns) = data;

        for (entity, mut clock, _myturn) in (&entities, &mut hunger_clock, &turns).join() {
            clock.duration -= 1;
            if clock.duration < 1 {
                match clock.state {
                    HungerState::WellFed => {
                        clock.state = HungerState::Normal;
                        clock.duration = 200;
                        if entity == *player_entity {
                            log.entries.push("Wieclaw nie jest juz najedzony.".to_string());
                        }
                    }
                    HungerState::Normal => {
                        clock.state = HungerState::Hungry;
                        clock.duration = 200;
                        if entity == *player_entity {
                            log.entries.push("Wieclaw jest glodny.".to_string());
                        }
                    }
                    HungerState::Hungry => {
                        clock.state = HungerState::Starving;
                        clock.duration = 200;
                        if entity == *player_entity {
                            log.entries.push("Wieclaw umiera z glodu!".to_string());
                        }
                    }
                    HungerState::Starving => {
                        if entity == *player_entity {
                            log.entries.push("Wieclaw odczuwa bolesny skurcz zoladka i traci 1 HP!".to_string());
                        }
                        SufferDamage::new_damage(&mut inflict_damage, entity, 1, false);
                    }
                }
            }
        }
    }
}