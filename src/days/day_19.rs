use std::{
    cell::RefCell,
    cmp::Ordering,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

pub fn main(input: &str) -> (u32, u32) {
    let blueprints: Vec<Blueprint> = Blueprint::transform_example(input)
        .lines()
        .map(|line| Blueprint::from_str(line).unwrap())
        .collect();

    dbg!(&blueprints);

    let max = RefCell::new(Inventory::default());
    test_blueprint(
        &max,
        &blueprints[1],
        Inventory::default(),
        Inventory {
            ore: 1,
            ..Inventory::default()
        },
        Inventory::default(),
        24,
    );
    let max = max.into_inner();
    dbg!(max);

    (0, 0)
}

fn test_blueprint(
    max: &RefCell<Inventory>,
    blueprint: &Blueprint,
    inventory: Inventory,
    robots: Inventory,
    new_robots: Inventory,
    minutes_remaining: u32,
) {
    if minutes_remaining == 0 {
        let current_max = max.borrow().clone();
        if inventory.geodes > current_max.geodes {
            dbg!(inventory, robots);
            max.replace(inventory);
        }
    } else {
        if minutes_remaining > 1 {
            if new_robots.is_empty() && blueprint.geode_robot_cost <= inventory {
                test_blueprint(
                    max,
                    blueprint,
                    inventory - blueprint.geode_robot_cost,
                    robots,
                    new_robots
                        + Inventory {
                            geodes: 1,
                            ..Default::default()
                        },
                    minutes_remaining,
                );
            }

            if new_robots.is_empty() && blueprint.obsidian_robot_cost <= inventory {
                test_blueprint(
                    max,
                    blueprint,
                    inventory - blueprint.obsidian_robot_cost,
                    robots,
                    new_robots
                        + Inventory {
                            obsidian: 1,
                            ..Default::default()
                        },
                    minutes_remaining,
                );
            }

            if new_robots.is_empty() && blueprint.clay_robot_cost <= inventory {
                test_blueprint(
                    max,
                    blueprint,
                    inventory - blueprint.clay_robot_cost,
                    robots,
                    new_robots
                        + Inventory {
                            clay: 1,
                            ..Default::default()
                        },
                    minutes_remaining,
                );
            }

            if new_robots.is_empty() && blueprint.ore_robot_cost <= inventory {
                test_blueprint(
                    max,
                    blueprint,
                    inventory - blueprint.ore_robot_cost,
                    robots,
                    new_robots
                        + Inventory {
                            ore: 1,
                            ..Default::default()
                        },
                    minutes_remaining,
                );
            }
        }

        test_blueprint(
            max,
            blueprint,
            inventory + robots,
            robots + new_robots,
            Inventory::default(),
            minutes_remaining - 1,
        );
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Inventory {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
}

impl Inventory {
    fn is_empty(self) -> bool {
        self.ore == 0 && self.clay == 0 && self.obsidian == 0 && self.geodes == 0
    }
}

impl Ord for Inventory {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if self.ore <= other.ore
            && self.clay <= other.clay
            && self.obsidian <= other.obsidian
            && self.geodes <= other.geodes
        {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Inventory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Inventory {
    type Output = Inventory;

    fn add(self, rhs: Self) -> Self::Output {
        Inventory {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geodes: self.geodes + rhs.geodes,
        }
    }
}

impl AddAssign for Inventory {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl SubAssign for Inventory {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Sub for Inventory {
    type Output = Inventory;

    fn sub(self, rhs: Self) -> Self::Output {
        Inventory {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geodes: self.geodes - rhs.geodes,
        }
    }
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: u32,
    ore_robot_cost: Inventory,
    clay_robot_cost: Inventory,
    obsidian_robot_cost: Inventory,
    geode_robot_cost: Inventory,
}

impl Blueprint {
    #[allow(unused)]
    fn transform_example(input: &str) -> String {
        input.replace("\n\n", "\n").replace("\n  ", " ")
    }
}

impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.split(". ");
        let mut first_chunk = chunks
            .next()
            .ok_or(format!("missing first chunk: {s}"))?
            .split(": ");
        let id_chunk = first_chunk.next().ok_or(format!("missing id chunk: {s}"))?;
        let id: u32 = id_chunk
            .split_whitespace()
            .skip(1)
            .next()
            .ok_or(format!("missing id: {s}"))?
            .parse()
            .map_err(|_| format!("invalid id: {s}"))?;

        let words = first_chunk
            .next()
            .ok_or(format!("missing ore robot chunk: {s}"))?
            .split_whitespace();
        let ore_robot_cost_ore: u32 = words
            .skip(4)
            .next()
            .ok_or(format!("missing ore robot cost: {s}"))?
            .parse()
            .map_err(|_| format!("invalid ore robot cost: {s}"))?;

        let words = chunks
            .next()
            .ok_or(format!("missing clay robot chunk: {s}"))?
            .split_whitespace();
        let clay_robot_cost_ore: u32 = words
            .skip(4)
            .next()
            .ok_or(format!("missing clay robot ore cost: {s}"))?
            .parse()
            .map_err(|_| format!("invalid ore robot cost: {s}"))?;

        let words = chunks
            .next()
            .ok_or(format!("missing obsidian robot chunk: {s}"))?
            .split_whitespace();
        let mut words = words.skip(4);
        let obsidian_robot_cost_ore: u32 = words
            .next()
            .ok_or(format!("missing obsidian robot cost: {s}"))?
            .parse()
            .map_err(|_| format!("invalid ore robot cost: {s}"))?;
        let obsidian_robot_cost_clay: u32 = words
            .skip(2)
            .next()
            .ok_or(format!("missing obsidian robot cost: {s}"))?
            .parse()
            .map_err(|_| format!("invalid ore robot cost: {s}"))?;

        let words = chunks
            .next()
            .ok_or(format!("missing geode robot chunk: {s}"))?
            .split_whitespace();
        let mut words = words.skip(4);
        let geode_robot_cost_ore: u32 = words
            .next()
            .ok_or(format!("missing geode robot cost: {s}"))?
            .parse()
            .map_err(|_| format!("invalid ore robot cost: {s}"))?;
        let geode_robot_cost_obsidian: u32 = words
            .skip(2)
            .next()
            .ok_or(format!("missing obsidian robot cost: {s}"))?
            .parse()
            .map_err(|_| format!("invalid ore robot cost: {s}"))?;

        Ok(Blueprint {
            id,
            ore_robot_cost: Inventory {
                ore: ore_robot_cost_ore,
                ..Default::default()
            },
            clay_robot_cost: Inventory {
                ore: clay_robot_cost_ore,
                ..Default::default()
            },
            obsidian_robot_cost: Inventory {
                ore: obsidian_robot_cost_ore,
                clay: obsidian_robot_cost_clay,
                ..Default::default()
            },
            geode_robot_cost: Inventory {
                ore: geode_robot_cost_ore,
                obsidian: geode_robot_cost_obsidian,
                ..Default::default()
            },
        })
    }
}
