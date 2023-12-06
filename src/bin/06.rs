advent_of_code::solution!(6);

#[derive(PartialEq, Debug)]
struct Race {
    time: f64,
    record: f64,
}

impl Race {
    fn get_solution(&self) -> u32 {
        let delta = self.time.powf(2f64) - 4f64 * self.record;
        let delta_squared = delta.sqrt();
        let x_one: f64 = (-1f64 * self.time + delta_squared) / -2f64;
        let x_two: f64 = (-1f64 * self.time - delta_squared) / -2f64;
        let correction = if x_two.fract() == 0.0 { 1i32 } else { 0 };
        let result = x_two.ceil() as i32 - correction - x_one.ceil() as i32;
        result as u32
    }
}

fn get_races(input: &str) -> Vec<Race> {
    let (raw_times, raw_records) = input.split_once("\n").unwrap();
    let times = raw_times.split_whitespace().skip(1).map(|t| t.parse::<f64>().unwrap());
    let records = raw_records.split_whitespace().skip(1).map(|r| r.parse::<f64>().unwrap());
    times
        .zip(records)
        .map(|(time, record)| Race { time, record })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    get_races(input)
        .iter()
        .map(|r| r.get_solution())
        .product::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let binding = input.replace(" ", "");
    let (raw_times, raw_records) = binding.split_once("\n").unwrap();
    let (_, time) = raw_times.split_once(":").unwrap();
    let (_, record) = raw_records.split_once(":").unwrap();
    let race = Race { time: time.parse::<f64>().unwrap(), record: record.parse::<f64>().unwrap() };
    Some(race.get_solution())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }

    #[test]
    fn test_get_races() {
        let result = get_races(&advent_of_code::template::read_file("examples", DAY));
        let expected = vec![Race { time: 7f64, record: 9f64 }, Race { time: 15f64, record: 40f64 }, Race { time: 30f64, record: 200f64 }];
        assert_eq!(result, expected);
    }
}
