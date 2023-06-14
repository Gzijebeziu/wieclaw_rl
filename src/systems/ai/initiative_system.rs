use specs::prelude::*;
use crate::{Initiative, Position, MyTurn, Attributes, RunState, Pools, Duration, EquipmentChanged, StatusEffect, DamageOverTime};

pub struct InitiativeSystem {}

impl<'a> System<'a> for InitiativeSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteStorage<'a, Initiative>,
                        ReadStorage<'a, Position>,
                        WriteStorage<'a, MyTurn>,
                        Entities<'a>,
                        ReadStorage<'a, Attributes>,
                        WriteExpect<'a, RunState>,
                        ReadExpect<'a, Entity>,
                        ReadExpect<'a, rltk::Point>,
                        ReadStorage<'a, Pools>,
                        WriteStorage<'a, Duration>,
                        WriteStorage<'a, EquipmentChanged>,
                        ReadStorage<'a, StatusEffect>,
                        ReadStorage<'a, DamageOverTime>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut initiatives, positions, mut turns, entities, attributes,
            mut runstate, player, player_pos, pools, mut durations, mut dirty, statuses, dots) = data;

        if *runstate != RunState::Ticking { return; }

        turns.clear();

        for (entity, initiative, pos) in (&entities, &mut initiatives, &positions).join() {
            initiative.current -= 1;
            if initiative.current < 1 {
                let mut myturn = true;

                initiative.current = 6 + crate::rng::roll_dice(1, 6);

                if let Some(attr) = attributes.get(entity) {
                    initiative.current -= attr.quickness.bonus;
                }

                if let Some(pools) = pools.get(entity) {
                    initiative.current += f32::floor(pools.total_initiative_penalty) as i32;
                }

                if entity == *player {
                    *runstate = RunState::AwaitingInput;
                } else {
                    let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_pos, rltk::Point::new(pos.x, pos.y));
                    if distance > 20.0 {
                        myturn = false;
                    }
                }

                if myturn {
                    turns.insert(entity, MyTurn{}).expect("Unable to insert turn");
                }
            }
        }

        if *runstate == RunState::AwaitingInput {
            use crate::effects::*;
            for (effect_entity, duration, status) in (&entities, &mut durations, &statuses).join() {
                if entities.is_alive(status.target) {
                    duration.turns -= 1;
                    if let Some(dot) = dots.get(effect_entity) {
                        add_effect(
                            None,
                            EffectType::Damage{ amount : dot.damage },
                            Targets::Single{ target : status.target }
                        );
                    }
                    if duration.turns < 1 {
                        dirty.insert(status.target, EquipmentChanged{}).expect("Unable to insert");
                        entities.delete(effect_entity).expect("Unable to delete");
                    }
                }
            }
        }
    }
}