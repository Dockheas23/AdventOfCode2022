use super::blueprint::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct State {
    pub minerals: u64,
    pub bots: u64,
}

impl State {
    pub fn new() -> Self {
        Self {
            minerals: 0,
            bots: 1,
        }
    }

    pub fn geodes(&self) -> u16 {
        (self.minerals >> 48 & 0xffff) as u16
    }

    pub fn geode_bots(&self) -> u16 {
        (self.bots >> 48 & 0xffff) as u16
    }

    pub fn obsidian(&self) -> u16 {
        (self.minerals >> 32 & 0xffff) as u16
    }

    pub fn obsidian_bots(&self) -> u16 {
        (self.bots >> 32 & 0xffff) as u16
    }

    pub fn clay(&self) -> u16 {
        (self.minerals >> 16 & 0xffff) as u16
    }

    pub fn clay_bots(&self) -> u16 {
        (self.bots >> 16 & 0xffff) as u16
    }

    pub fn ore(&self) -> u16 {
        (self.minerals & 0xffff) as u16
    }

    pub fn ore_bots(&self) -> u16 {
        (self.bots & 0xffff) as u16
    }

    pub fn mine(&self) -> u64 {
        self.bots
    }

    pub fn store(&mut self, new_minerals: u64) {
        self.minerals += new_minerals;
    }

    pub fn affordable_bots(&self, blueprint: &Blueprint) -> Vec<BotType> {
        let mut result = Vec::new();
        if self.can_afford_ore_bot(blueprint) {
            result.push(BotType::Ore);
        }
        if self.can_afford_clay_bot(blueprint) {
            result.push(BotType::Clay);
        }
        if self.can_afford_obsidian_bot(blueprint) {
            result.push(BotType::Obsidian);
        }
        if self.can_afford_geode_bot(blueprint) {
            result.push(BotType::Geode);
        }
        result
    }

    pub fn can_afford_geode_bot(&self, blueprint: &Blueprint) -> bool {
        self.ore() >= blueprint.geode.0 && self.obsidian() >= blueprint.geode.1
    }

    pub fn can_afford_obsidian_bot(&self, blueprint: &Blueprint) -> bool {
        self.ore() >= blueprint.obsidian.0 && self.clay() >= blueprint.obsidian.1
    }

    pub fn can_afford_clay_bot(&self, blueprint: &Blueprint) -> bool {
        self.ore() >= blueprint.clay.0
    }

    pub fn can_afford_ore_bot(&self, blueprint: &Blueprint) -> bool {
        self.ore() >= blueprint.ore.0
    }

    pub fn time_to_next_ore_bot(&self, blueprint: &Blueprint) -> u32 {
        let ore_needed = blueprint.ore.0 as i32 - self.ore() as i32;
        if ore_needed <= 0 {
            0
        } else {
            (ore_needed as f32 / self.ore_bots() as f32).ceil() as u32
        }
    }

    pub fn time_to_next_clay_bot(&self, blueprint: &Blueprint) -> u32 {
        let ore_needed = blueprint.clay.0 as i32 - self.ore() as i32;
        if ore_needed <= 0 {
            0
        } else {
            (ore_needed as f32 / self.ore_bots() as f32).ceil() as u32
        }
    }

    pub fn time_to_next_obsidian_bot(&self, blueprint: &Blueprint) -> u32 {
        let ore_needed = blueprint.obsidian.0 as i32 - self.ore() as i32;
        let clay_needed = blueprint.obsidian.1 as i32 - self.clay() as i32;
        if ore_needed <= 0 && clay_needed <= 0 {
            0
        } else if self.clay_bots() == 0 {
            u32::MAX
        } else {
            std::cmp::max(
                (ore_needed as f32 / self.ore_bots() as f32).ceil() as u32,
                (clay_needed as f32 / self.clay_bots() as f32).ceil() as u32,
            )
        }
    }

    pub fn time_to_next_geode_bot(&self, blueprint: &Blueprint) -> u32 {
        let ore_needed = blueprint.geode.0 as i32 - self.ore() as i32;
        let obsidian_needed = blueprint.geode.1 as i32 - self.obsidian() as i32;
        if ore_needed <= 0 && obsidian_needed <= 0 {
            0
        } else if self.obsidian_bots() == 0 {
            u32::MAX
        } else {
            std::cmp::max(
                (ore_needed as f32 / self.ore_bots() as f32).ceil() as u32,
                (obsidian_needed as f32 / self.obsidian_bots() as f32).ceil() as u32,
            )
        }
    }

    pub fn with_ore_bot(&self, blueprint: &Blueprint) -> State {
        let mut state = self.clone();
        state.build_ore_bot(blueprint);
        state
    }

    pub fn with_clay_bot(&self, blueprint: &Blueprint) -> State {
        let mut state = self.clone();
        state.build_clay_bot(blueprint);
        state
    }

    pub fn with_obsidian_bot(&self, blueprint: &Blueprint) -> State {
        let mut state = self.clone();
        state.build_obsidian_bot(blueprint);
        state
    }

    pub fn with_geode_bot(&self, blueprint: &Blueprint) -> State {
        let mut state = self.clone();
        state.build_geode_bot(blueprint);
        state
    }

    pub fn build_geode_bot(&mut self, blueprint: &Blueprint) {
        let GeodeRobot(ore_cost, obsidian_cost) = blueprint.geode;
        if ore_cost > (self.minerals & 0xffff) as u16
            || obsidian_cost > (self.minerals >> 32 & 0xffff) as u16
        {
            panic!(
                "Can't afford a geode bot [state={:?}][blueprint={:?}]",
                self, blueprint
            );
        }
        self.minerals -= ore_cost as u64;
        self.minerals -= (obsidian_cost as u64) << 32;
        self.bots += 1 << 48;
    }

    pub fn build_obsidian_bot(&mut self, blueprint: &Blueprint) {
        let ObsidianRobot(ore_cost, clay_cost) = blueprint.obsidian;
        if ore_cost > (self.minerals & 0xffff) as u16
            || clay_cost > (self.minerals >> 16 & 0xffff) as u16
        {
            panic!(
                "Can't afford an obsidian bot [state={:?}][blueprint={:?}]",
                self, blueprint
            );
        }
        self.minerals -= ore_cost as u64;
        self.minerals -= (clay_cost as u64) << 16;
        self.bots += 1 << 32;
    }

    pub fn build_clay_bot(&mut self, blueprint: &Blueprint) {
        let ClayRobot(ore_cost) = blueprint.clay;
        if ore_cost > (self.minerals & 0xffff) as u16 {
            panic!(
                "Can't afford a clay bot [state={:?}][blueprint={:?}]",
                self, blueprint
            );
        }
        self.minerals -= ore_cost as u64;
        self.bots += 1 << 16;
    }

    pub fn build_ore_bot(&mut self, blueprint: &Blueprint) {
        let OreRobot(ore_cost) = blueprint.ore;
        if ore_cost > (self.minerals & 0xffff) as u16 {
            panic!(
                "Can't afford an ore bot [state={:?}][blueprint={:?}]",
                self, blueprint
            );
        }
        self.minerals -= ore_cost as u64;
        self.bots += 1;
    }

    pub fn is_strictly_better_than(&self, other: &State) -> bool {
        self != other
            && self.ore_bots() >= other.ore_bots()
            && self.clay_bots() >= other.clay_bots()
            && self.obsidian_bots() >= other.obsidian_bots()
            && self.geode_bots() >= other.geode_bots()
            && self.ore() >= other.ore()
            && self.clay() >= other.clay()
            && self.obsidian() >= other.obsidian()
            && self.geodes() >= other.geodes()
    }
}
