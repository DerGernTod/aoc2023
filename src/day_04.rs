use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[allow(dead_code)]
pub fn day_04() {
    println!("Sum of points is {}", calc_sum_points("./input/day_04.txt"));
    println!(
        "Number of won scratch cards: {}",
        calc_num_owned_scratchcards("./input/day_04.txt")
    );
}

fn calc_sum_points(path: &str) -> u32 {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(strip_game_id)
        .map(get_num_winning_numbers)
        .filter(did_win)
        .map(calc_points_for_num_wins)
        .sum()
}

fn strip_game_id(line: &str) -> &str {
    line.split(": ").last().unwrap()
}

fn get_num_winning_numbers(line: &str) -> u32 {
    let mut line_iter = line.split(" | ").map(str_to_num_list);
    let winning_numbers: HashSet<u32> = line_iter.next().unwrap().collect();
    line_iter
        .next()
        .unwrap()
        .filter(move |num| winning_numbers.contains(num))
        .count()
        .try_into()
        .unwrap()
}

fn str_to_num_list(str: &str) -> impl Iterator<Item = u32> + '_ {
    str.split(" ").flat_map(|str| str.trim().parse())
}

fn did_win(num_wins: &u32) -> bool {
    return num_wins != &0;
}

fn calc_points_for_num_wins(num_wins: u32) -> u32 {
    2u32.pow(num_wins.saturating_sub(1u32))
}

fn calc_num_owned_scratchcards(path: &str) -> u32 {
    let num_wins_per_game: Vec<u32> = fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(strip_game_id)
        .map(get_num_winning_numbers)
        .collect();

    let mut scratch_cards_per_game: HashMap<usize, u32> = HashMap::new();
    num_wins_per_game
        .iter()
        .enumerate()
        .rev()
        .map(|(game_id, num_wins)| {
            let won_scratch_cards = 1
                + (1..=(*num_wins as usize))
                    .map(|i| i + game_id)
                    .flat_map(|id| scratch_cards_per_game.get(&id))
                    .sum::<u32>();
            scratch_cards_per_game.insert(game_id, won_scratch_cards);
            won_scratch_cards
        })
        .sum()
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
        assert_eq!(calc_num_owned_scratchcards("./input_test/day_04.txt"), 30);
    }
}
