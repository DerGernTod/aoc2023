use std::{borrow::Borrow, fs};
#[derive(Debug)]
struct MapRange {
    destination: i64,
    source: i64,
    range: i64,
}
impl MapRange {
    fn new(destination: i64, source: i64, range: i64) -> MapRange {
        MapRange {
            destination,
            source,
            range,
        }
    }
    fn calc_dest(&self, source: i64) -> Option<i64> {
        let offset = source - self.source;
        if offset < self.range && offset >= 0 {
            Some(self.destination + offset)
        } else {
            None
        }
    }
}

impl FromIterator<i64> for MapRange {
    fn from_iter<T: IntoIterator<Item = i64>>(iter: T) -> Self {
        let mut i64_iter = iter.into_iter();
        MapRange::new(
            i64_iter.next().unwrap(),
            i64_iter.next().unwrap(),
            i64_iter.next().unwrap(),
        )
    }
}

#[allow(dead_code)]
pub fn day_05() {
    let min_location = determine_lowest_location("./input/day_05.txt");
    println!("Lowest location is {}", min_location);
    let min_location = determine_lowest_location_seed_range("./input/day_05.txt");
    println!("Lowest location with seed range is {}", min_location);
}

fn determine_lowest_location(path: &str) -> i64 {
    let input_string = fs::read_to_string(path).unwrap();
    let seeds_iter = read_seeds(&input_string);
    let category_maps = read_maps(&input_string);

    calc_min_location_for_seeds(seeds_iter, &category_maps)
}

fn determine_lowest_location_seed_range(path: &str) -> i64 {
    let input_string = fs::read_to_string(path).unwrap();
    let seeds_iter = read_seeds_range(&input_string);

    let category_maps = read_maps(&input_string);
    calc_min_location_for_seeds(seeds_iter, &category_maps)
}

fn read_seeds_range(input_string: &String) -> impl Iterator<Item = i64> + '_ {
    input_string
        .split("\n\n")
        .take(1)
        .map(|line| {
            let mut seed_ranges = line.split(" ").skip(1).flat_map(|num| num.parse());
            let mut seeds = vec![];
            while let (Some(index), Some(range)) = (seed_ranges.next(), seed_ranges.next()) {
                for num in index..index + range {
                    seeds.push(num);
                }
            }
            seeds
        })
        .flatten()
}
fn extract_value(closure: impl FnOnce() -> Vec<i64>) -> Vec<i64> {
    closure()
}
fn read_seeds(input_string: &String) -> impl Iterator<Item = i64> + '_ {
    input_string
        .split("\n\n")
        .take(1)
        .map(|line| line.split(" ").skip(1).flat_map(|num| num.parse()))
        .flatten()
}

fn read_maps(input_string: &String) -> Vec<Vec<MapRange>> {
    input_string
        .split("\n\n")
        .skip(1)
        .map(|category_lines| {
            category_lines
                .split("\n")
                .skip(1)
                .map(|line| line.split(" ").flat_map(|num| num.parse()).collect())
                .collect()
        })
        .collect()
}

fn calc_min_location_for_seeds(
    seeds_iter: impl Iterator<Item = i64>,
    category_maps: &Vec<Vec<MapRange>>,
) -> i64 {
    let min_location = seeds_iter
        .map(|seed| {
            let mut input = seed;
            for map_category in category_maps {
                input = map_category
                    .iter()
                    .find_map(|map_range| map_range.calc_dest(input))
                    .unwrap_or(input);
            }
            input
        })
        .min()
        .unwrap();

    min_location
}

#[cfg(test)]
mod tests {
    use crate::day_05::*;
    #[test]
    fn test_part_1() {
        let min_location = determine_lowest_location("./input_test/day_05.txt");
        assert_eq!(min_location, 35);
    }
    #[test]
    fn test_part_2() {
        let min_location = determine_lowest_location_seed_range("./input_test/day_05.txt");
        assert_eq!(min_location, 46);
    }
    #[test]
    fn test_calc_dest() {
        let map_vec = vec![MapRange::new(50, 98, 2), MapRange::new(52, 50, 48)];
        assert_eq!(map_vec[0].calc_dest(79), None);
        assert_eq!(map_vec[1].calc_dest(79), Some(81));
    }
}
