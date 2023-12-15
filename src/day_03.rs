use std::{collections::{HashMap, HashSet}, fs};

pub fn day_03() {
    println!("sum of machine parts is {}", calc_part_number_sum("./input/day_03.txt"));
}

fn calc_part_number_sum(path: &str) -> u32 {
    let matrix = read_to_matrix(path);
    matrix
        .iter()
        .filter(|(_, char)| !char.is_digit(10) && char != && '.')
        .flat_map(|(coords, _)| find_surrounding_digit_coords(coords, &matrix))
        .collect::<HashSet<(i32, i32)>>()
        .iter()
        .map(|coords| map_coord_to_number(coords, &matrix))
        .collect::<HashMap<(i32, i32), u32>>()
        .values()
        .inspect(|num| println!("got num {:?}", num))
        .sum()
}

fn read_to_matrix(path: &str) -> HashMap<(i32, i32), char> {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .enumerate()
        .flat_map(|(index_y, line)| line.char_indices().map(move |(index_x, char)| ((index_x as i32, index_y as i32), char)))
        .collect()
}

fn find_surrounding_digit_coords((x, y): &(i32, i32), matrix: &HashMap<(i32, i32), char>) -> Vec<(i32, i32)> {
    let mut digit_coords = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            if (i == 0 && j == 0) {
                continue;
            }
            let check_x = x + i;
            let check_y = y + j;
            if let Some(ch) = matrix.get(&(check_x, check_y)) {
                if ch.is_digit(10) {
                    digit_coords.push((check_x, check_y));
                }
            }
        }
    }
    digit_coords
}

fn map_coord_to_number((x, y): &(i32, i32), matrix: &HashMap<(i32, i32), char>) -> ((i32, i32), u32) {
    let mut numbers = Vec::new();
    numbers.push(matrix.get(&(*x, *y)).unwrap().to_digit(10).unwrap());
    let mut left_most = (*x, *y);
    
    let mut prev = (x - 1, *y);
    while let Some(prev_char) = matrix.get(&prev) {
        if let Some(digit) = prev_char.to_digit(10) {
            numbers.insert(0, digit);
            left_most = prev;
        } else {
            break;
        }
        prev = (prev.0 - 1, *y);
    }

    let mut next = (x + 1, *y);
    while let Some(next_char) = matrix.get(&next) {
        if let Some(digit) = next_char.to_digit(10) {
            numbers.push(digit);
        } else {
            break;
        }
        next = (next.0 + 1, *y);
    }

    let num = numbers
        .iter()
        .rev()
        .enumerate()
        .map(|(index, digit)| 10u32.pow(index as u32) * digit)
        .sum();
    (left_most, num)
}

#[cfg(test)]
mod tests {
    use crate::day_03::*;
    
    #[test]
    fn test_part_1() {
        assert_eq!(calc_part_number_sum("./input_test/day_03.txt"), 4361)
    }
    #[test]
    fn test_part_2() {
    }
}