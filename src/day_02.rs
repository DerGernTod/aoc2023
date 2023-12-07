use std::fs;

const LIMIT_RED: usize = 12;
const LIMIT_GREEN: usize = 13;
const LIMIT_BLUE: usize = 14;

#[allow(dead_code)]
pub fn day_02() {
    println!("Sum of valid ids: {}", calc_sum_valid_game_ids("./input/day_02.txt"));
    println!("Power of cube games: {}", calc_sum_cube_powers("./input/day_02.txt"));
}

fn calc_sum_valid_game_ids(path: &str) -> usize {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .enumerate()
        .filter(|(_, line)| are_all_cubes_within_limit(line))
        .map(|(index, _) | index + 1)
        .sum()
}

fn are_all_cubes_within_limit(line: &str) -> bool {
    !line
        .split("; ")
        .map(|round| round.split(", "))
        .flatten()
        .any(is_cube_above_limit)
}

fn is_cube_above_limit(cube: &str) -> bool {
    let cubes_str: Vec<&str> = cube.split(": ").last().unwrap().split(" ").take(2).collect();
    let cube_num: usize = cubes_str.get(0).unwrap().parse().unwrap();
    match cubes_str.get(1) {
        Some(&"red") => {
            cube_num > LIMIT_RED
        },
        Some(&"blue") => {
            cube_num > LIMIT_BLUE
        },
        Some(&"green") => {
            cube_num > LIMIT_GREEN
        },
        _ => panic!("Didn't find valid cube count!")
    }
}

fn calc_sum_cube_powers(path: &str) -> usize {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|game| game.split(": ").last().unwrap())
        .map(|rounds| {
            let cube_counts = get_cube_counts(rounds);
            get_color_max(&cube_counts, "green")
            * get_color_max(&cube_counts, "blue")
            * get_color_max(&cube_counts, "red")
        })
        .sum()
}

fn get_cube_counts(rounds: &str) -> Vec<(usize, &str)> {
    rounds
        .split("; ")
        .map(|round| round.split(", "))
        .flatten()
        .map(|cubes| {
            let mut spl = cubes.split(" ");
            (spl.next().unwrap().parse().unwrap(), spl.next().unwrap())
        })
        .collect()
}

fn get_color_max(cube_counts: &Vec<(usize, &str)>, color: &str) -> usize {
    cube_counts
        .iter()
        .filter_map(|(num, col)| (col == &color).then_some(*num))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day_02::*;
    
    #[test]
    fn test_part_1() {
        let sum = calc_sum_valid_game_ids("./input_test/day_02.txt");
        assert_eq!(sum, 8);
    }
    #[test]
    fn test_part_2() {
        let sum = calc_sum_cube_powers("./input_test/day_02.txt");
        assert_eq!(sum, 2286);
    }
}