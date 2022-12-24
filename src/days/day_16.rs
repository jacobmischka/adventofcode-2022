use std::collections::{BTreeSet, HashMap, HashSet};

#[cfg(feature = "multiprocessing")]
use std::sync::{Arc, RwLock};

#[cfg(feature = "multiprocessing")]
use rayon::prelude::*;

pub fn main(input: &str) -> (u32, u32) {
    let mut flows: HashMap<&str, u32> = HashMap::new();
    let mut tunnels: HashMap<&str, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let mut chunks = line.trim().split("; ");
        let mut words = chunks.next().unwrap().split_whitespace().skip(1);
        let valve = words.next().unwrap();
        flows.insert(
            valve,
            words
                .skip(2)
                .next()
                .unwrap()
                .split('=')
                .skip(1)
                .next()
                .unwrap()
                .replace(';', "")
                .parse()
                .unwrap(),
        );

        let words = chunks.next().unwrap().split_whitespace().skip(4);
        tunnels.insert(valve, words.map(|word| word.replace(',', "")).collect());
    }

    let alone_max = max_pressure(
        &flows,
        &tunnels,
        BTreeSet::new(),
        30,
        ValvePosition {
            valve: "AA",
            prev: "",
        },
        None,
    );

    let with_elephant_max = max_pressure(
        &flows,
        &tunnels,
        BTreeSet::new(),
        26,
        ValvePosition {
            valve: "AA",
            prev: "",
        },
        Some(ValvePosition {
            valve: "AA",
            prev: "",
        }),
    );

    (alone_max, with_elephant_max)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ValvePosition<'input> {
    prev: &'input str,
    valve: &'input str,
}

fn get_adjacency_matrix<'input>(
    flows: &'input HashMap<&'input str, u32>,
    tunnels: &'input HashMap<&'input str, Vec<String>>,
) -> HashMap<&'input str, HashMap<&'input str, u32>> {
    let mut adjacency_matrix: HashMap<&'input str, HashMap<&'input str, u32>> = HashMap::new();
    let valves_with_flows: Vec<&'input str> = flows
        .iter()
        .filter_map(|(valve, flow)| if *flow > 0 { Some(*valve) } else { None })
        .collect();

    for source in tunnels.keys().copied() {
        let mut dist: HashMap<&'input str, u32> = HashMap::new();
        let mut prev: HashMap<&'input str, &'input str> = HashMap::new();
        let mut queue: HashSet<&'input str> = HashSet::new();

        for valve in tunnels.keys().copied() {
            dist.insert(valve, u32::MAX);
            queue.insert(valve);
        }
        dist.insert(source, 0);

        while !queue.is_empty() {
            let (valve, valve_dist) = queue.iter().copied().fold(("", u32::MAX), |acc, valve| {
                let valve_dist = *dist.get(valve).unwrap();
                if valve_dist < acc.1 {
                    (valve, valve_dist)
                } else {
                    acc
                }
            });

            queue.remove(valve);

            for neighbor in tunnels.get(valve).unwrap() {
                let neighbor = neighbor.as_str();
                if !queue.contains(neighbor) {
                    continue;
                }

                let alt = valve_dist + 1;
                if alt < *dist.get(neighbor).unwrap() {
                    dist.insert(neighbor, alt);
                    prev.insert(neighbor, valve);
                }
            }
        }

        for dest in valves_with_flows.iter().copied() {
            let mut dist = 0;
            let mut u = Some(dest);

            while let Some(valve) = u {
                dist += 1;
                u = prev.get(valve).copied();
            }

            adjacency_matrix
                .entry(source)
                .or_default()
                .insert(dest, dist);
        }
    }

    adjacency_matrix
}

type Cache<'input> = HashMap<(BTreeSet<&'input str>, u32, &'input str, Option<&'input str>), u32>;

#[cfg(feature = "multiprocessing")]
fn max_pressure<'input>(
    flows: &HashMap<&'input str, u32>,
    tunnels: &'input HashMap<&'input str, Vec<String>>,
    open: BTreeSet<&'input str>,
    time_remaining: u32,
    you: ValvePosition<'input>,
    elephant: Option<ValvePosition<'input>>,
) -> u32 {
    let adjacency_matrix = get_adjacency_matrix(&flows, &tunnels);

    let closed: BTreeSet<&str> = flows
        .iter()
        .filter_map(|(valve, flow)| {
            if *flow > 0 && !open.contains(valve) {
                Some(*valve)
            } else {
                None
            }
        })
        .collect();

    max_pressure_inner(
        Arc::new(RwLock::new(HashMap::new())),
        flows,
        tunnels,
        &adjacency_matrix,
        closed,
        time_remaining,
        you,
        elephant,
    )
}

#[cfg(feature = "multiprocessing")]
fn max_pressure_inner<'input>(
    cache: Arc<RwLock<Cache<'input>>>,
    flows: &HashMap<&'input str, u32>,
    tunnels: &'input HashMap<&'input str, Vec<String>>,
    adjacency_matrix: &'input HashMap<&'input str, HashMap<&'input str, u32>>,
    closed: BTreeSet<&'input str>,
    time_remaining: u32,
    you: ValvePosition<'input>,
    elephant: Option<ValvePosition<'input>>,
) -> u32 {
    let mut max = 0;

    if time_remaining == 1 || closed.is_empty() {
        return max;
    }

    let cache_key = if let Some(elephant) = elephant {
        let min = you.valve.min(elephant.valve);
        let max = you.valve.max(elephant.valve);
        (closed.clone(), time_remaining, min, Some(max))
    } else {
        (closed.clone(), time_remaining, you.valve, None)
    };

    if let Ok(cache) = cache.read() {
        if let Some(val) = cache.get(&cache_key) {
            return *val;
        }
    }

    let consider_you = |cache: Arc<RwLock<Cache<'input>>>,
                        closed: BTreeSet<&'input str>,
                        elephant: Option<ValvePosition<'input>>| {
        let your_flow = *flows.get(you.valve).unwrap();
        let mut max = 0;

        if your_flow > 0 && closed.contains(you.valve) {
            let mut new_closed = closed.clone();
            new_closed.remove(you.valve);
            max = max.max(
                your_flow * (time_remaining - 1)
                    + max_pressure_inner(
                        Arc::clone(&cache),
                        flows,
                        tunnels,
                        adjacency_matrix,
                        new_closed,
                        time_remaining - 1,
                        ValvePosition {
                            valve: you.valve,
                            prev: you.valve,
                        },
                        elephant,
                    ),
            );
        }

        if closed.iter().copied().all(|closed| {
            *adjacency_matrix
                .get(you.valve)
                .unwrap()
                .get(closed)
                .unwrap()
                > time_remaining
        }) {
            return max;
        }

        let your_connected_valves = tunnels.get(you.valve).unwrap();
        max = max.max(
            your_connected_valves
                .par_iter()
                .fold(
                    || max,
                    |max, valve| {
                        if valve == you.prev || Some(valve.as_str()) == elephant.map(|e| e.valve) {
                            return max;
                        }

                        max.max(max_pressure_inner(
                            Arc::clone(&cache),
                            flows,
                            tunnels,
                            adjacency_matrix,
                            closed.clone(),
                            time_remaining - 1,
                            ValvePosition {
                                valve,
                                prev: you.valve,
                            },
                            elephant,
                        ))
                    },
                )
                .reduce(|| max, |acc, val| acc.max(val)),
        );

        max
    };

    if let Some(elephant) = elephant {
        let elephant_flow = *flows.get(elephant.valve).unwrap();

        if elephant_flow > 0 && closed.contains(elephant.valve) {
            let mut new_closed = closed.clone();
            new_closed.remove(elephant.valve);
            max = max.max(
                elephant_flow * (time_remaining - 1)
                    + consider_you(
                        Arc::clone(&cache),
                        new_closed,
                        Some(ValvePosition {
                            valve: elephant.valve,
                            prev: elephant.valve,
                        }),
                    ),
            );
        }

        if closed.iter().copied().all(|closed| {
            *adjacency_matrix
                .get(elephant.valve)
                .unwrap()
                .get(closed)
                .unwrap()
                > time_remaining
        }) {
            max = max.max(consider_you(
                Arc::clone(&cache),
                closed,
                Some(ValvePosition {
                    valve: elephant.valve,
                    prev: elephant.valve,
                }),
            ))
        } else {
            let elephant_connected_valves = tunnels.get(elephant.valve).unwrap();
            max = max.max(
                elephant_connected_valves
                    .par_iter()
                    .fold(
                        || max,
                        |max, valve| {
                            if valve == elephant.prev {
                                return max;
                            }

                            max.max(consider_you(
                                Arc::clone(&cache),
                                closed.clone(),
                                Some(ValvePosition {
                                    valve,
                                    prev: elephant.valve,
                                }),
                            ))
                        },
                    )
                    .reduce(|| max, |acc, val| acc.max(val)),
            );
        }
    } else {
        max = max.max(consider_you(Arc::clone(&cache), closed, None));
    }

    if let Ok(mut cache) = cache.write() {
        cache
            .entry(cache_key)
            .and_modify(|prev| {
                if max >= *prev {
                    *prev = max;
                }
            })
            .or_insert(max);
    }

    max
}

#[cfg(not(feature = "multiprocessing"))]
fn max_pressure<'input>(
    flows: &HashMap<&'input str, u32>,
    tunnels: &'input HashMap<&'input str, Vec<String>>,
    open: BTreeSet<&'input str>,
    time_remaining: u32,
    you: ValvePosition<'input>,
    elephant: Option<ValvePosition<'input>>,
) -> u32 {
    let mut cache = HashMap::new();
    max_pressure_inner(
        &mut cache,
        flows,
        tunnels,
        open,
        time_remaining,
        you,
        elephant,
    )
}

#[cfg(not(feature = "multiprocessing"))]
fn max_pressure_inner<'input>(
    cache: &mut Cache<'input>,
    flows: &HashMap<&'input str, u32>,
    tunnels: &'input HashMap<&'input str, Vec<String>>,
    open: BTreeSet<&'input str>,
    time_remaining: u32,
    you: ValvePosition<'input>,
    elephant: Option<ValvePosition<'input>>,
) -> u32 {
    let mut max = 0;

    if time_remaining == 0 || open.len() == flows.len() {
        return max;
    }

    if let Some(val) = cache.get(&(
        open.clone(),
        time_remaining,
        you.valve,
        elephant.map(|e| e.valve),
    )) {
        return *val;
    }

    let mut consider_you = |open: BTreeSet<&'input str>,
                            elephant: Option<ValvePosition<'input>>| {
        let mut max = 0;
        let your_flow = *flows.get(you.valve).unwrap();

        if your_flow > 0 && !open.contains(you.valve) {
            let mut new_open = open.clone();
            new_open.insert(you.valve);
            max = max.max(
                your_flow * (time_remaining - 1)
                    + max_pressure_inner(
                        cache,
                        flows,
                        tunnels,
                        new_open,
                        time_remaining - 1,
                        ValvePosition {
                            valve: you.valve,
                            prev: you.valve,
                        },
                        elephant,
                    ),
            );
        }

        let your_connected_valves = tunnels.get(you.valve).unwrap();
        for valve in your_connected_valves {
            if valve == you.prev {
                continue;
            }

            max = max.max(max_pressure_inner(
                cache,
                flows,
                tunnels,
                open.clone(),
                time_remaining - 1,
                ValvePosition {
                    valve,
                    prev: you.valve,
                },
                elephant,
            ));
        }

        max
    };

    if let Some(elephant) = elephant {
        let elephant_flow = *flows.get(elephant.valve).unwrap();

        if elephant_flow > 0 && !open.contains(elephant.valve) {
            let mut new_open = open.clone();
            new_open.insert(elephant.valve);
            max = max.max(
                elephant_flow * (time_remaining - 1)
                    + consider_you(
                        new_open,
                        Some(ValvePosition {
                            valve: elephant.valve,
                            prev: elephant.valve,
                        }),
                    ),
            );
        }

        let elephant_connected_valves = tunnels.get(elephant.valve).unwrap();
        for valve in elephant_connected_valves {
            if valve == elephant.prev {
                continue;
            }

            max = max.max(consider_you(
                open.clone(),
                Some(ValvePosition {
                    valve,
                    prev: elephant.valve,
                }),
            ));
        }
    } else {
        max = max.max(consider_you(open.clone(), None));
    }

    let prev = cache.insert(
        (open, time_remaining, you.valve, elephant.map(|e| e.valve)),
        max,
    );

    if prev.is_some() {
        dbg!(&max, &prev);
    }

    assert_eq!(prev, None);

    max
}
