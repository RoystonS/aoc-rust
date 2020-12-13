use std::vec::Vec;

pub struct BusInfo {
    departure: u64,
    services: Vec<Service>,
}
pub struct Service {
    offset: u64,
    interval: u64,
}

#[aoc_generator(day13)]
pub fn parser(input: &str) -> BusInfo {
    let lines = input.lines().collect::<Vec<_>>();

    let services = lines[1]
        .split(',')
        .enumerate()
        .filter_map(|(index, record)| {
            if let Ok(id) = record.parse::<u64>() {
                Some(Service {
                    interval: id,
                    offset: index as u64,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    BusInfo {
        departure: lines[0].parse().unwrap(),
        services,
    }
}

#[aoc(day13, part1)]
pub fn day13_part1(data: &BusInfo) -> u64 {
    let departure = data.departure;

    let result = data
        .services
        .iter()
        .fold((0, u64::MAX), |(best_bus, best_wait), service| {
            let next_arrival = smallest_multiple_above(service.interval, departure);
            let wait = next_arrival - departure;
            if wait < best_wait {
                (service.interval, wait)
            } else {
                (best_bus, best_wait)
            }
        });

    match result {
        (best_bus, best_wait) => best_bus * best_wait,
    }
}

fn smallest_multiple_above(multiplier: u64, minimum: u64) -> u64 {
    let factor = minimum / multiplier;
    if factor * multiplier == minimum {
        minimum
    } else {
        multiplier * (factor + 1)
    }
}

#[aoc(day13, part2)]
pub fn day13_part2(data: &BusInfo) -> u64 {
    let (_, offset) =
        data.services
            .iter()
            .fold((1, 0), |(combined_interval, combined_offset), svc| {
                // For a bus a repeating every 6 and another bus repeating every 4, the overall cycle will repeat every 12 (the LCM)
                let new_combined_interval = lcm(combined_interval, svc.interval);

                // Find the offset that satisfies the previous offset and the new bus
                let mut multiplier = 0;
                loop {
                    let possible_offset = combined_interval * multiplier + combined_offset;

                    if (possible_offset + svc.offset) % svc.interval == 0 {
                        return (new_combined_interval, possible_offset);
                    }

                    multiplier += 1;
                }
            });

    offset
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
