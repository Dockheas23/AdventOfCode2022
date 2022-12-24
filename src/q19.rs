mod blueprint;
mod state;
mod strategy;

use crate::parser::*;
use blueprint::*;
use state::State;
use std::collections::HashSet;
use std::io;
use strategy::*;

const DEBUG: bool = false;
const INFO: bool = true;
const INPUT_FILE: &str = "input/input19.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input19_sample.txt";

impl TryFrom<&mut FileLines> for Blueprint {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let line = lines.next_result()?;
        let (id, l1) = line
            .trim_start_matches("Blueprint ")
            .split_once(": Each ore robot costs ")
            .unwrap();
        let (ore, l2) = l1.split_once(" ore. Each clay robot costs ").unwrap();
        let (clay, l3) = l2.split_once(" ore. Each obsidian robot costs ").unwrap();
        let (obsidian_ore, l4) = l3.split_once(" ore and ").unwrap();
        let (obsidian_clay, l5) = l4.split_once(" clay. Each geode robot costs ").unwrap();
        let (geode_ore, l6) = l5.split_once(" ore and ").unwrap();
        let geode_obsidian = l6.trim_end_matches(" obsidian.");
        Ok(Blueprint {
            id: id.parse().unwrap(),
            ore: OreRobot(ore.parse().unwrap()),
            clay: ClayRobot(clay.parse().unwrap()),
            obsidian: ObsidianRobot(
                obsidian_ore.parse().unwrap(),
                obsidian_clay.parse().unwrap(),
            ),
            geode: GeodeRobot(geode_ore.parse().unwrap(), geode_obsidian.parse().unwrap()),
        })
    }
}

impl Iterator for FileInput<FileLines, Blueprint> {
    type Item = Blueprint;

    fn next(&mut self) -> Option<Blueprint> {
        parse_from(&mut self.source)
    }
}

pub fn part_1() -> io::Result<u32> {
    const MINUTES: u32 = 24;
    let blueprints = parse_lines::<Blueprint>(INPUT_FILE)?;
    Ok(blueprints.map(|b| quality_level(&b, MINUTES)).sum())
}

fn _part_1_sample() -> io::Result<u32> {
    const MINUTES: u32 = 24;
    let blueprints = parse_lines::<Blueprint>(_INPUT_FILE_SAMPLE)?;
    Ok(blueprints.map(|b| quality_level(&b, MINUTES)).sum())
}

pub fn part_2() -> io::Result<u64> {
    const MINUTES: u32 = 32;
    let blueprints = parse_lines::<Blueprint>(INPUT_FILE)?;
    Ok(blueprints
        .take(3)
        .map(|b| max_geodes(&b, MINUTES) as u64)
        .product())
}

fn _part_2_sample() -> io::Result<u64> {
    const MINUTES: u32 = 32;
    let blueprints = parse_lines::<Blueprint>(_INPUT_FILE_SAMPLE)?;
    Ok(blueprints
        .take(3)
        .map(|b| max_geodes(&b, MINUTES) as u64)
        .product())
}

fn _debug_state(state: &State, blueprint: &Blueprint) {
    if DEBUG {
        print!(
            "Bots: ({}, {}, {}, {}) ",
            state.ore_bots(),
            state.clay_bots(),
            state.obsidian_bots(),
            state.geode_bots()
        );
        print!(
            "Minerals: ({}, {}, {}, {}) ",
            state.ore(),
            state.clay(),
            state.obsidian(),
            state.geodes()
        );
        println!(
            "Times: ({}, {}, {}, {})",
            state.time_to_next_ore_bot(blueprint),
            state.time_to_next_clay_bot(blueprint),
            state.time_to_next_obsidian_bot(blueprint),
            state.time_to_next_geode_bot(blueprint)
        );
    }
}

fn _debug(s: &str) {
    if DEBUG {
        println!("{}", s);
    }
}

fn _info(s: &str) {
    if INFO {
        println!("{}", s);
    }
}

fn quality_level(blueprint: &Blueprint, minutes: u32) -> u32 {
    blueprint.id as u32 * max_geodes(blueprint, minutes) as u32
}

fn max_geodes(blueprint: &Blueprint, minutes: u32) -> u16 {
    let mut states: HashSet<State> = HashSet::new();
    states.insert(State::new());
    let mut strategy = ValidBotStrategy(minutes);

    _debug(format!("Blueprint {:?}", blueprint).as_str());
    for t in 0..minutes {
        _debug(format!("After time t={:2}:", t + 1).as_str());
        let mut next_states: HashSet<State> = HashSet::new();
        for state in states {
            let new_minerals = state.mine();
            for mut new_state in strategy.apply(&state, blueprint, t) {
                new_state.store(new_minerals);
                next_states.insert(new_state);
                _debug_state(&new_state, blueprint);
            }
        }
        states = prune_states(&next_states);
        _debug("After pruning...");
        for pruned in states.iter() {
            _debug_state(&pruned, blueprint);
        }
        _debug("---");
    }
    let best_state = states.iter().max_by_key(|s| s.geodes()).unwrap();
    _info(format!("Best: Geodes {}", best_state.geodes()).as_str());
    best_state.geodes()
}

fn prune_states(states: &HashSet<State>) -> HashSet<State> {
    states
        .iter()
        .filter(|s| !states.iter().any(|s2| s2.is_strictly_better_than(s)))
        .map(|&s| s)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::state::State;
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_state_equal() {
        let mut s1 = State::new();
        let mut s2 = State::new();
        s1.store(0xff);
        assert_ne!(s1, s2);
        s2.store(0xff);
        assert_eq!(s1, s2);
        assert_eq!((1, "hey", 3), (1, "hey", 3));
        assert_eq!(State::new(), State::new());
    }

    #[test]
    fn test_part_1() {
        assert_eq!(1404, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(33, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(5880, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(3472, _part_2_sample().unwrap());
    }
}
