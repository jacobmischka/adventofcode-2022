use std::collections::{BTreeSet, HashMap};

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

    let mut cache = HashMap::new();

    let alone_max = max_pressure(
        &mut cache,
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

    let mut cache = HashMap::new();
    let with_elephant_max = max_pressure(
        &mut cache,
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

fn max_pressure<'input>(
    cache: &mut HashMap<
        (
            BTreeSet<&'input str>,
            u32,
            u32,
            &'input str,
            Option<&'input str>,
        ),
        u32,
    >,
    flows: &HashMap<&'input str, u32>,
    tunnels: &'input HashMap<&'input str, Vec<String>>,
    open: BTreeSet<&'input str>,
    current_pressure: u32,
    time_remaining: u32,
    you: ValvePosition<'input>,
    elephant: Option<ValvePosition<'input>>,
) -> u32 {
    let mut max = current_pressure;

    if time_remaining == 0 || open.len() == flows.len() {
        return max;
    }

    if let Some(val) = cache.get(&(
        open.clone(),
        current_pressure,
        time_remaining,
        you.valve,
        elephant.map(|e| e.valve),
    )) {
        return *val;
    }

    let mut consider_you = |open: BTreeSet<&'input str>,
                            current_pressure: u32,
                            elephant: Option<ValvePosition<'input>>| {
        let your_flow = *flows.get(you.valve).unwrap();

        if your_flow > 0 && !open.contains(you.valve) {
            let mut new_open = open.clone();
            new_open.insert(you.valve);
            max = max.max(max_pressure(
                cache,
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
            ));
        }

        let your_connected_valves = tunnels.get(you.valve).unwrap();
        for valve in your_connected_valves {
            if valve == you.prev {
                continue;
            }

            max = max.max(max_pressure(
                cache,
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
            ));
        }
    };

    if let Some(elephant) = elephant {
        let elephant_flow = *flows.get(elephant.valve).unwrap();

        if elephant_flow > 0 && !open.contains(elephant.valve) {
            let mut new_open = open.clone();
            new_open.insert(elephant.valve);
            consider_you(
                new_open,
                current_pressure + elephant_flow * (time_remaining - 1),
                Some(ValvePosition {
                    valve: elephant.valve,
                    prev: elephant.valve,
                }),
            );
        }

        let elephant_connected_valves = tunnels.get(elephant.valve).unwrap();
        for valve in elephant_connected_valves {
            if valve == elephant.prev {
                continue;
            }

            consider_you(
                open.clone(),
                current_pressure,
                Some(ValvePosition {
                    valve,
                    prev: elephant.valve,
                }),
            );
        }
    } else {
        consider_you(open.clone(), current_pressure, None);
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

    max
}
