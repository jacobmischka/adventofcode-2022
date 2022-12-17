use rayon::prelude::*;
use std::collections::{BTreeSet, HashMap};

pub fn main(input: &str) -> (u32, u32) {
    let mut flows: HashMap<&str, u32> = HashMap::new();
    let mut tunnels: HashMap<&str, Vec<&str>> = HashMap::new();

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
        tunnels.insert(
            valve,
            words.map(|word| word.trim_end_matches(',')).collect(),
        );
    }

    let (alone_max, _) = max_pressure(
        HashMap::new(),
        &flows,
        &tunnels,
        BTreeSet::new(),
        0,
        30,
        ValvePosition {
            valve: "AA",
            prev: "",
        },
        None,
    );

    let (with_elephant_max, _) = max_pressure(
        HashMap::new(),
        &flows,
        &tunnels,
        BTreeSet::new(),
        0,
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

type Cache<'input> = HashMap<
    (
        BTreeSet<&'input str>,
        u32,
        u32,
        &'input str,
        Option<&'input str>,
    ),
    u32,
>;

fn max_pressure<'input>(
    mut cache: Cache<'input>,
    flows: &HashMap<&'input str, u32>,
    tunnels: &'input HashMap<&'input str, Vec<&'input str>>,
    open: BTreeSet<&'input str>,
    current_pressure: u32,
    time_remaining: u32,
    you: ValvePosition<'input>,
    elephant: Option<ValvePosition<'input>>,
) -> (u32, Cache<'input>) {
    let mut max = current_pressure;

    if time_remaining == 0 || open.len() == flows.len() {
        return (max, cache);
    }

    if let Some(val) = cache.get(&(
        open.clone(),
        current_pressure,
        time_remaining,
        you.valve,
        elephant.map(|e| e.valve),
    )) {
        return (*val, cache);
    }

    let consider_you = |mut cache: Cache<'input>,
                        open: BTreeSet<&'input str>,
                        current_pressure: u32,
                        elephant: Option<ValvePosition<'input>>| {
        let mut you_max = 0;
        let your_flow = *flows.get(you.valve).unwrap();

        if your_flow > 0 && !open.contains(you.valve) {
            let mut new_open = open.clone();
            new_open.insert(you.valve);
            let (new_max, new_cache) = max_pressure(
                cache.clone(),
                flows,
                tunnels,
                new_open,
                current_pressure + your_flow * (time_remaining - 1),
                time_remaining - 1,
                ValvePosition {
                    valve: you.valve,
                    prev: you.valve,
                },
                elephant,
            );
            you_max = you_max.max(new_max);
            for (k, v) in new_cache {
                cache.insert(k, v);
            }
        }

        let your_connected_valves = tunnels.get(you.valve).unwrap();
        let (you_max, you_cache) = your_connected_valves
            .as_slice()
            .par_iter()
            .fold(
                || (you_max, cache.clone()),
                |(acc, mut cache), valve| {
                    if *valve == you.prev {
                        return (acc, cache);
                    }

                    let (new_max, new_cache) = max_pressure(
                        cache.clone(),
                        flows,
                        tunnels,
                        open.clone(),
                        current_pressure,
                        time_remaining - 1,
                        ValvePosition {
                            valve,
                            prev: you.valve,
                        },
                        elephant,
                    );

                    for (k, v) in new_cache {
                        cache.insert(k, v);
                    }

                    (acc.max(new_max), cache)
                },
            )
            .reduce(
                || (you_max, cache.clone()),
                |(acc, mut cache), (val, new_cache)| {
                    for (k, v) in new_cache {
                        cache.insert(k, v);
                    }
                    (acc.max(val), cache)
                },
            );

        (you_max, you_cache)
    };

    if let Some(elephant) = elephant {
        let elephant_flow = *flows.get(elephant.valve).unwrap();

        if elephant_flow > 0 && !open.contains(elephant.valve) {
            let mut new_open = open.clone();
            new_open.insert(elephant.valve);
            let (new_max, new_cache) = consider_you(
                cache.clone(),
                new_open,
                current_pressure + elephant_flow * (time_remaining - 1),
                Some(ValvePosition {
                    valve: elephant.valve,
                    prev: elephant.valve,
                }),
            );
            for (k, v) in new_cache {
                cache.insert(k, v);
            }
            max = max.max(new_max);
        }

        let elephant_connected_valves = tunnels.get(elephant.valve).unwrap();
        let (new_max, new_cache) = elephant_connected_valves
            .par_iter()
            .fold(
                || (max, cache.clone()),
                |(acc, mut cache), valve| {
                    if *valve == elephant.prev {
                        return (acc, cache);
                    }

                    let (new_max, new_cache) = consider_you(
                        cache.clone(),
                        open.clone(),
                        current_pressure,
                        Some(ValvePosition {
                            valve,
                            prev: elephant.valve,
                        }),
                    );

                    for (k, v) in new_cache {
                        cache.insert(k, v);
                    }

                    (acc.max(new_max), cache)
                },
            )
            .reduce(
                || (max, cache.clone()),
                |(acc, mut cache), (val, new_cache)| {
                    for (k, v) in new_cache {
                        cache.insert(k, v);
                    }
                    (acc.max(val), cache)
                },
            );

        max = max.max(new_max);
        for (k, v) in new_cache {
            cache.insert(k, v);
        }
    } else {
        consider_you(cache.clone(), open.clone(), current_pressure, None);
    }

    let prev = cache.insert(
        (
            open,
            current_pressure,
            time_remaining,
            you.valve,
            elephant.map(|e| e.valve),
        ),
        max,
    );

    if prev.is_some() {
        dbg!(&max, &prev);
    }

    assert_eq!(prev, None);

    (max, cache)
}
