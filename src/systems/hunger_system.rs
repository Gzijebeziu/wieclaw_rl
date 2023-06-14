use specs::prelude::*;
use crate::{HungerClock, RunState, HungerState, MyTurn, effects::{add_effect, EffectType, Targets}};

pub struct HungerSystem {}

impl<'a> System<'a> for HungerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
                        Entities<'a>,
                        WriteStorage<'a, HungerClock>,
                        ReadExpect<'a, Entity>,
                        ReadExpect<'a, RunState>,
                        ReadStorage<'a, MyTurn>
                    );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut hunger_clock, player_entity, _runstate, turns) = data;

        for (entity, mut clock, _myturn) in (&entities, &mut hunger_clock, &turns).join() {
            clock.duration -= 1;
            if clock.duration < 1 {
                match clock.state {
                    HungerState::WellFed => {
                        clock.state = HungerState::Normal;
                        clock.duration = 200;
                        if entity == *player_entity {
                            crate::gamelog::Logger::new()
                                .color(rltk::ORANGE)
                                .append("Wieclaw nie jest juz najedzony.")
                                .log();
                        }
                    }
                    HungerState::Normal => {
                        clock.state = HungerState::Hungry;
                        clock.duration = 200;
                        if entity == *player_entity {
                            crate::gamelog::Logger::new()
                                .color(rltk::ORANGE)
                                .append("Wieclaw jest glodny.")
                                .log();
                        }
                    }
                    HungerState::Hungry => {
                        clock.state = HungerState::Starving;
                        clock.duration = 200;
                        if entity == *player_entity {
                            crate::gamelog::Logger::new()
                                .color(rltk::RED)
                                .append("Wieclaw umiera z glodu!")
                                .log();
                        }
                    }
                    HungerState::Starving => {
                        if entity == *player_entity {
                            crate::gamelog::Logger::new()
                                .color(rltk::RED)
                                .append("Wieclaw odczuwa bolesny skurcz zoladka i traci 1 HP!")
                                .log();
                        }
                        add_effect(
                            None,
                            EffectType::Damage{ amount: 1 },
                            Targets::Single{ target: entity }
                        );
                    }
                }
            }
        }
    }
}