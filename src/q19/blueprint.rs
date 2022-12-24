#[derive(Debug)]
pub struct OreRobot(pub u16);
#[derive(Debug)]
pub struct ClayRobot(pub u16);
#[derive(Debug)]
pub struct ObsidianRobot(pub u16, pub u16);
#[derive(Debug)]
pub struct GeodeRobot(pub u16, pub u16);

#[derive(Debug, PartialEq)]
pub enum BotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
pub struct Blueprint {
    pub id: u16,
    pub ore: OreRobot,
    pub clay: ClayRobot,
    pub obsidian: ObsidianRobot,
    pub geode: GeodeRobot,
}
