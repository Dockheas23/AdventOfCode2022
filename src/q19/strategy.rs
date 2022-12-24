use super::blueprint::*;
use super::state::State;
use std::collections::HashSet;

pub trait Strategy {
    fn name(&self) -> String;
    fn apply(&mut self, state: &State, blueprint: &Blueprint, time: u32) -> HashSet<State>;
}

#[derive(Debug)]
pub struct ValidBotStrategy(pub u32);

impl ValidBotStrategy {
    fn should_include_ore_bot(&self, state: &State, blueprint: &Blueprint) -> bool {
        let max_ore = [
            blueprint.ore.0,
            blueprint.clay.0,
            blueprint.obsidian.0,
            blueprint.geode.0,
        ]
        .into_iter()
        .max()
        .unwrap();
        state.can_afford_ore_bot(blueprint)
            && state.ore_bots() < max_ore
            && state
                .with_ore_bot(blueprint)
                .time_to_next_geode_bot(blueprint)
                <= state.time_to_next_geode_bot(blueprint)
    }

    fn should_include_clay_bot(&self, state: &State, blueprint: &Blueprint) -> bool {
        state.can_afford_clay_bot(blueprint)
            && state.clay_bots() < blueprint.obsidian.1
            && state
                .with_clay_bot(blueprint)
                .time_to_next_geode_bot(blueprint)
                <= state.time_to_next_geode_bot(blueprint)
    }

    fn should_include_obsidian_bot(&self, state: &State, blueprint: &Blueprint) -> bool {
        state.can_afford_obsidian_bot(blueprint)
            && state.clay_bots() > 0
            && state.obsidian_bots() < blueprint.geode.1
            && state
                .with_obsidian_bot(blueprint)
                .time_to_next_geode_bot(blueprint)
                <= state.time_to_next_geode_bot(blueprint)
    }
}

impl Strategy for ValidBotStrategy {
    fn apply(&mut self, state: &State, blueprint: &Blueprint, _time: u32) -> HashSet<State> {
        if state.affordable_bots(blueprint).is_empty() {
            return HashSet::from([*state]);
        } else if state.can_afford_geode_bot(blueprint) {
            return HashSet::from([state.with_geode_bot(blueprint)]);
        }

        let mut result = HashSet::from([*state]);
        if self.should_include_ore_bot(state, blueprint) {
            result.insert(state.with_ore_bot(blueprint));
        }
        if self.should_include_clay_bot(state, blueprint) {
            result.insert(state.with_clay_bot(blueprint));
        }
        if self.should_include_obsidian_bot(state, blueprint) {
            result.insert(state.with_obsidian_bot(blueprint));
        }
        result
    }

    fn name(&self) -> String {
        String::from(format!("ValidBotStrategy({})", self.0))
    }
}
