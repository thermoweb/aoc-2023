use std::collections::HashMap;
use std::ops::Range;

use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(5);

type RangeMap = Range<u64>;

#[derive(Debug)]
struct Convertor {
    input_range: Range<u64>,
    output_range: u64,
}

impl Convertor {
    fn get(&self, source: u64) -> Option<u64> {
        if self.input_range.contains(&source) {
            let value = self.output_range + source - self.input_range.start;
            return Some(value);
        }
        None
    }

    fn get_convertor(map: &str) -> Convertor {
        let split = map.split_whitespace().collect::<Vec<_>>();
        let source_start = split[1].parse::<u64>().unwrap();
        let target_start = split[0].parse::<u64>().unwrap();
        let range_end = source_start + split[2].parse::<u64>().unwrap();
        Convertor {
            input_range: source_start..range_end,
            output_range: target_start,
        }
    }
}

#[derive(Debug)]
struct ConvertorMap {
    convertors: Vec<Convertor>,
}

impl ConvertorMap {
    fn get(&self, source: u64) -> Option<u64> {
        let convertor = self
            .convertors
            .iter()
            .find(|c| c.input_range.contains(&source));
        if let Some(convertor) = convertor {
            return convertor.get(source);
        }
        Some(source)
    }

    fn get_convertors(map: &str) -> (&str, ConvertorMap) {
        let mut convertors = vec![];
        let (raw_name, values) = map.split_once('\n').unwrap();
        let (name, _) = raw_name.split_once(' ').unwrap();
        values.lines().for_each(|l| {
            convertors.push(Convertor::get_convertor(l));
        });
        (name, ConvertorMap { convertors })
    }

    fn translate(&self, ranges: Vec<RangeMap>) -> Vec<RangeMap> {
        let mut output = vec![];

        ranges.iter().for_each(|input_range| {
            let mut output_ranges: Vec<RangeMap> = vec![];
            let mut covered_ranges: Vec<RangeMap> = vec![];
            let mut uncovered_ranges: Vec<RangeMap> = vec![];

            for convertor in &self.convertors {
                if convertor.input_range.start < input_range.end
                    && convertor.input_range.end > input_range.start
                {
                    let start_range = convertor.input_range.start.max(input_range.start);
                    let end_range = convertor.input_range.end.min(input_range.end);
                    let total_items = (start_range..end_range).count() as u64;

                    let mut gap = 0;
                    if start_range > convertor.input_range.start {
                        gap = start_range - convertor.input_range.start;
                    }

                    covered_ranges.push(start_range..end_range);
                    output_ranges.push(RangeMap {
                        start: convertor.output_range + gap,
                        end: convertor.output_range + gap + total_items,
                    });
                }
            }

            if covered_ranges.is_empty() {
                output.push(input_range.to_owned());
            } else {
                covered_ranges.sort_by_key(|r| r.start);
                let mut current_range = &covered_ranges[0];
                if input_range.start != current_range.start {
                    uncovered_ranges.push(RangeMap {
                        start: input_range.start,
                        end: current_range.end - 1,
                    });
                }

                for next_range in covered_ranges.iter().skip(1) {
                    if current_range.end < next_range.start {
                        output_ranges.push(RangeMap {
                            start: current_range.end,
                            end: next_range.start,
                        });
                        current_range = next_range;
                    }
                }

                if current_range.end < input_range.end {
                    output_ranges.push(RangeMap {
                        start: current_range.end,
                        end: input_range.end,
                    });
                }
            }

            output.extend(output_ranges);
            output.extend(uncovered_ranges);
        });

        output
    }
}

fn get_seeds(str: &str) -> Vec<RangeMap> {
    static PART_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+ \d+)").unwrap());
    let mut ranges = vec![];
    PART_REG.find_iter(str).for_each(|m| {
        // println!("{:?}", &str[m.range()]);
        let (start, end) = &str[m.range()].split_once(' ').unwrap();
        let range_start = start.parse::<u64>().unwrap();
        let range_end = range_start + end.parse::<u64>().unwrap();
        ranges.push(range_start..range_end);
    });
    ranges
}

pub fn part_one(input: &str) -> Option<u32> {
    let (raw_init, maps) = input.split_once("\n\n").unwrap();
    let (_, init) = raw_init.split_once(':').unwrap();
    let mut convertors_map = HashMap::new();
    maps.split("\n\n").for_each(|m| {
        let (name, map) = ConvertorMap::get_convertors(m);
        convertors_map.insert(name, map);
    });
    init.split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .map(|seed| {
            convertors_map
                .get("seed-to-soil")
                .unwrap()
                .get(seed)
                .unwrap()
        })
        .map(|seed| {
            convertors_map
                .get("soil-to-fertilizer")
                .unwrap()
                .get(seed)
                .unwrap()
        })
        .map(|seed| {
            convertors_map
                .get("fertilizer-to-water")
                .unwrap()
                .get(seed)
                .unwrap()
        })
        .map(|seed| {
            convertors_map
                .get("water-to-light")
                .unwrap()
                .get(seed)
                .unwrap()
        })
        .map(|seed| {
            convertors_map
                .get("light-to-temperature")
                .unwrap()
                .get(seed)
                .unwrap()
        })
        .map(|seed| {
            convertors_map
                .get("temperature-to-humidity")
                .unwrap()
                .get(seed)
                .unwrap()
        })
        .map(|seed| {
            convertors_map
                .get("humidity-to-location")
                .unwrap()
                .get(seed)
                .unwrap()
        })
        .map(|r| r as u32)
        .min()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (raw_init, maps) = input.split_once("\n\n").unwrap();
    let (_, init) = raw_init.split_once(':').unwrap();
    let seeds = get_seeds(init);

    let mut convertors_map = HashMap::new();
    maps.split("\n\n").for_each(|m| {
        let (name, map) = ConvertorMap::get_convertors(m);
        convertors_map.insert(name, map);
    });

    let soil = convertors_map.get("seed-to-soil").unwrap().translate(seeds);
    let fert = convertors_map
        .get("soil-to-fertilizer")
        .unwrap()
        .translate(soil);
    let water = convertors_map
        .get("fertilizer-to-water")
        .unwrap()
        .translate(fert);
    let light = convertors_map
        .get("water-to-light")
        .unwrap()
        .translate(water);
    let temp = convertors_map
        .get("light-to-temperature")
        .unwrap()
        .translate(light);
    let hum = convertors_map
        .get("temperature-to-humidity")
        .unwrap()
        .translate(temp);
    let location = convertors_map
        .get("humidity-to-location")
        .unwrap()
        .translate(hum);
    location.iter().map(|r| r.start as u32).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_convertor() {
        let convertor = Convertor {
            output_range: 10,
            input_range: 1..9,
        };
        let result = convertor.get(2);
        assert_eq!(result, Some(11))
    }
}
