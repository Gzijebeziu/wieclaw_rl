use specs::prelude::*;
use crate::{MyTurn, Confusion, RunState, StatusEffect, effects::{add_effect, EffectType, Targets}};
use std::collections::HashSet;

pub struct TurnStatusSystem {}

impl<'a> System<'a> for TurnStatusSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteStorage<'a, MyTurn>,
                        WriteStorage<'a, Confusion>,
                        Entities<'a>,
                        ReadExpect<'a, RunState>,
                        ReadStorage<'a, StatusEffect>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut turns, confusion, entities, runstate, statuses) = data;

        if *runstate != RunState::Ticking { return; }

        let mut entity_turns = HashSet::new();
        for (entity, _turn) in (&entities, &turns).join() {
            entity_turns.insert(entity);
        }

        let mut not_my_turn : Vec<Entity> = Vec::new();
        for (effect_entity, status_effect) in (&entities, &statuses).join() {
            if entity_turns.contains(&status_effect.target) {
                if confusion.get(effect_entity).is_some() {
                    add_effect(
                        None,
                        EffectType::Particle{
                            glyph : rltk::to_cp437('?'),
                            fg : rltk::RGB::named(rltk::MAGENTA),
                            bg : rltk::RGB::named(rltk::BLACK),
                            lifespan : 200.0
                        },
                        Targets::Single{ target: status_effect.target }
                    );
                    not_my_turn.push(status_effect.target);
                }
            }
        }

        for e in not_my_turn {
            turns.remove(e);
        }
    }
}