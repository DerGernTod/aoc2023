use std::{collections::HashSet, fs};

pub fn day_04() {
    println!("Sum of points is {}", calc_sum_points("./input/day_04.txt"));
}

fn calc_sum_points(path: &str) -> u32 {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .flat_map(|line| line.split(": ").last())
        .map(get_num_winning_numbers)
        .sum()
}

fn get_num_winning_numbers(line: &str) -> u32 {
    let mut line_iter = line.split(" | ").map(str_to_num_list);
    let winning_numbers: HashSet<u32> = line_iter.next().unwrap().collect();
    let num_winning: u32 = line_iter
        .next()
        .unwrap()
        .filter(|num| winning_numbers.contains(num))
        .count()
        .try_into()
        .unwrap();
    if num_winning == 0 {
        0
    } else {
        2u32.pow(num_winning.saturating_sub(1u32))
    }
}

fn str_to_num_list(str: &str) -> impl Iterator<Item = u32> + '_ {
    str.split(" ").flat_map(|str| str.trim().parse())
}

#[cfg(test)]
mod tests {
    use crate::day_04::*;

    #[test]
    fn test_part_1() {
        assert_eq!(calc_sum_points("./input_test/day_04.txt"), 13);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            "a b  c".split(" ").collect::<Vec<&'static str>>(),
            vec!["a", "b", "c"]
        );
    }
}
