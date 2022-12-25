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
        30,
        ValvePosition {
            valve: "AA",
            open_time_end: 0,
        },
        None,
    );

    let with_elephant_max = max_pressure(
        &flows,
        &tunnels,
        26,
        ValvePosition {
            valve: "AA",
            open_time_end: 0,
        },
        Some(ValvePosition {
            valve: "AA",
            open_time_end: 0,
        }),
    );

    (alone_max, with_elephant_max)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ValvePosition<'input> {
    valve: &'input str,
    open_time_end: u32,
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

type Cache<'input> = HashMap<
    (
        BTreeSet<&'input str>,
        u32,
        ValvePosition<'input>,
        Option<ValvePosition<'input>>,
    ),
    u32,
>;

#[cfg(feature = "multiprocessing")]
fn max_pressure<'input>(
    flows: &HashMap<&'input str, u32>,
    tunnels: &'input HashMap<&'input str, Vec<String>>,
    time_remaining: u32,
    you: ValvePosition<'input>,
    elephant: Option<ValvePosition<'input>>,
) -> u32 {
    let adjacency_matrix = get_adjacency_matrix(&flows, &tunnels);

    let closed: BTreeSet<&str> = flows
        .iter()
        .filter_map(|(valve, flow)| if *flow > 0 { Some(*valve) } else { None })
        .collect();

    max_pressure_inner(
        Arc::new(RwLock::new(HashMap::new())),
        flows,
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
    adjacency_matrix: &'input HashMap<&'input str, HashMap<&'input str, u32>>,
    closed: BTreeSet<&'input str>,
    total_time: u32,
    you: ValvePosition<'input>,
    elephant: Option<ValvePosition<'input>>,
) -> u32 {
    let cache_key = (closed.clone(), total_time, you, elephant);

    if let Ok(cache) = cache.read() {
        if let Some(val) = cache.get(&cache_key) {
            return *val;
        }
    }

    let consider_you = |cache: Arc<RwLock<Cache<'input>>>,
                        closed: BTreeSet<&'input str>,
                        elephant: Option<ValvePosition<'input>>| {
        adjacency_matrix
            .get(you.valve)
            .unwrap()
            .par_iter()
            .filter(|(valve, distance)| {
                closed.contains(*valve) && you.open_time_end + *distance < total_time
            })
            .fold(
                || 0,
                |max, (valve, distance)| {
                    let mut new_closed = closed.clone();
                    new_closed.remove(valve);
                    let open_time_end = you.open_time_end + distance;
                    let time_remaining = total_time - open_time_end;
                    let flow = flows.get(valve).unwrap();

                    max.max(
                        flow * time_remaining
                            + max_pressure_inner(
                                Arc::clone(&cache),
                                flows,
                                adjacency_matrix,
                                new_closed,
                                total_time,
                                ValvePosition {
                                    valve,
                                    open_time_end,
                                },
                                elephant,
                            ),
                    )
                },
            )
            .reduce(|| 0, |acc, val| acc.max(val))
    };

    let max = if let Some(elephant) = elephant {
        adjacency_matrix
            .get(elephant.valve)
            .unwrap()
            .par_iter()
            .filter(|(valve, distance)| {
                closed.contains(*valve) && elephant.open_time_end + *distance < total_time
            })
            .fold(
                || 0,
                |max, (valve, distance)| {
                    let mut new_closed = closed.clone();
                    new_closed.remove(valve);
                    let open_time_end = elephant.open_time_end + distance;
                    let time_remaining = total_time - open_time_end;
                    let flow = flows.get(valve).unwrap();

                    max.max(
                        flow * time_remaining
                            + consider_you(
                                Arc::clone(&cache),
                                new_closed,
                                Some(ValvePosition {
                                    valve,
                                    open_time_end,
                                }),
                            ),
                    )
                },
            )
            .reduce(|| 0, |acc, val| acc.max(val))
    } else {
        consider_you(Arc::clone(&cache), closed, None)
    };

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
    time_remaining: u32,
    you: ValvePosition<'input>,
    elephant: Option<ValvePosition<'input>>,
) -> u32 {
    let adjacency_matrix = get_adjacency_matrix(&flows, &tunnels);

    let closed: BTreeSet<&str> = flows
        .iter()
        .filter_map(|(valve, flow)| if *flow > 0 { Some(*valve) } else { None })
        .collect();

    let mut cache = HashMap::new();

    max_pressure_inner(
        &mut cache,
        flows,
        &adjacency_matrix,
        closed,
        time_remaining,
        you,
        elephant,
    )
}

#[cfg(not(feature = "multiprocessing"))]
fn max_pressure_inner<'input>(
    cache: &mut Cache<'input>,
    flows: &HashMap<&'input str, u32>,
    adjacency_matrix: &'input HashMap<&'input str, HashMap<&'input str, u32>>,
    closed: BTreeSet<&'input str>,
    total_time: u32,
    you: ValvePosition<'input>,
    elephant: Option<ValvePosition<'input>>,
) -> u32 {
    let cache_key = (closed.clone(), total_time, you, elephant);

    if let Some(val) = cache.get(&cache_key) {
        return *val;
    }

    let consider_you = |cache: &mut Cache<'input>,
                        closed: BTreeSet<&'input str>,
                        elephant: Option<ValvePosition<'input>>| {
        adjacency_matrix
            .get(you.valve)
            .unwrap()
            .iter()
            .filter(|(valve, distance)| {
                closed.contains(*valve) && you.open_time_end + *distance < total_time
            })
            .fold(0, |max, (valve, distance)| {
                let mut new_closed = closed.clone();
                new_closed.remove(valve);
                let open_time_end = you.open_time_end + distance;
                let time_remaining = total_time - open_time_end;
                let flow = flows.get(valve).unwrap();

                max.max(
                    flow * time_remaining
                        + max_pressure_inner(
                            cache,
                            flows,
                            adjacency_matrix,
                            new_closed,
                            total_time,
                            ValvePosition {
                                valve,
                                open_time_end,
                            },
                            elephant,
                        ),
                )
            })
    };

    let max = if let Some(elephant) = elephant {
        adjacency_matrix
            .get(elephant.valve)
            .unwrap()
            .iter()
            .filter(|(valve, distance)| {
                closed.contains(*valve) && elephant.open_time_end + *distance < total_time
            })
            .fold(0, |max, (valve, distance)| {
                let mut new_closed = closed.clone();
                new_closed.remove(valve);
                let open_time_end = elephant.open_time_end + distance;
                let time_remaining = total_time - open_time_end;
                let flow = flows.get(valve).unwrap();

                max.max(
                    flow * time_remaining
                        + consider_you(
                            cache,
                            new_closed,
                            Some(ValvePosition {
                                valve,
                                open_time_end,
                            }),
                        ),
                )
            })
    } else {
        consider_you(cache, closed, None)
    };

    cache
        .entry(cache_key)
        .and_modify(|prev| {
            if max >= *prev {
                *prev = max;
            }
        })
        .or_insert(max);

    max
}
