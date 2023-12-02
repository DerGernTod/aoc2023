use std::fs;

const NUMBERS_TEXT: &'static [&'static str] = &[
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];

pub fn day_01() {
    let sum_numbers = form_numbers_of_digits_and_calc_sum("./input/day_01.txt");
    println!("sum of numbers: {}", sum_numbers);
    let sum_numbers_with_text = form_numbers_of_digits_and_text_and_calc_sum("./input/day_01.txt");
    println!("sum of numbers with text: {}", sum_numbers_with_text);
}

fn form_numbers_of_digits_and_calc_sum(path: &str) -> u32 {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|line| line.chars())
        .map(|chars_of_line| {
            let first = chars_of_line.to_owned().find_map(|char| char.to_digit(10)).unwrap();
            let last = chars_of_line.rev().find_map(|char| char.to_digit(10)).unwrap();
            first * 10 + last
        })
        .sum()
}

fn form_numbers_of_digits_and_text_and_calc_sum(path: &str) -> usize {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(line_to_two_digit_number)
        .sum()
}

fn line_to_two_digit_number(line: &str) -> usize {
    let (num_first, num_last) = get_numeric_digits(line);
    let (text_first, text_last) = get_text_digits(line);
    let digit_indices = vec![num_first, num_last, text_first, text_last];
    pick_digits_and_combine_to_num(digit_indices)
}

fn get_numeric_digits(line: &str) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let first_digit = line.char_indices().find_map(get_index_and_digit);
    let last_digit = line.char_indices().rev().find_map(get_index_and_digit);
    (first_digit, last_digit)
}

fn get_index_and_digit((index, char): (usize, char)) -> Option<(usize, usize)> {
    char.to_digit(10).and_then(|num| Some((index, usize::try_from(num).unwrap())))
}

fn get_text_digits(line: &str) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let mut found_indices = NUMBERS_TEXT
        .iter()
        .enumerate()
        .flat_map(|(num_index, num_str)| find_text_digit_indices(num_index, num_str, line))
        .collect::<Vec<(usize, usize)>>();
    found_indices.sort_by_key(|(index, _)| *index);
    let first = found_indices.first().copied();
    let last = found_indices.last().copied();
    (first, last)
}

fn find_text_digit_indices(num_index: usize, num_str: &str, line: &str) -> impl Iterator<Item = (usize, usize)> {
    let digit_of_found_text = num_index + 1;
    vec![
        line.find(num_str).and_then(|line_index| Some((line_index, digit_of_found_text))),
        line.rfind(num_str).and_then(|line_index| Some((line_index, digit_of_found_text)))
    ]
    .into_iter()
    .flatten()
}

fn pick_digits_and_combine_to_num(digit_indices: Vec<Option<(usize, usize)>>) -> usize {
    let mut collection_vec: Vec<&(usize, usize)> = digit_indices.iter().flatten().collect();
    collection_vec.sort_by_key(|(index, _)| *index);
    collection_vec.first().unwrap().1 * 10 + collection_vec.last().unwrap().1
}

#[cfg(test)]
mod tests {
    use crate::day_01::*;

    #[test]
    fn test_part_1() {
        let sum = form_numbers_of_digits_and_calc_sum("./input_test/day_01.txt");
        assert_eq!(sum, 142);
    }

    #[test]
    fn test_part_2() {
        let sum = form_numbers_of_digits_and_text_and_calc_sum("./input_test/day_01_b.txt");
        assert_eq!(sum, 281);
    }

    #[test]
    fn text_get_text_digits() {
        assert_eq!(get_text_digits("two1nine"), (Some((0, 2)), Some((4, 9))));
        assert_eq!(get_text_digits("eighttwothree"), (Some((0, 8)), Some((8, 3))));
        assert_eq!(get_text_digits("twofivefourb5four"), (Some((0, 2)), Some((13, 4))));
    }

    #[test]
    fn test_line_to_two_digit_number() {
        assert_eq!(line_to_two_digit_number("five3onelxjninenine45"), 55);
        assert_eq!(line_to_two_digit_number("six9mnfjmtsf2kfmznkxntninesevenrpmfjfpgsk"), 67);
        assert_eq!(line_to_two_digit_number("9vkrmbpnine5two5cbktwo6"), 96);
        assert_eq!(line_to_two_digit_number("one1bdr6"), 16);
        assert_eq!(line_to_two_digit_number("ksvctznmffourtwovbb9four5five"), 45);
        assert_eq!(line_to_two_digit_number("6nfhcklxlkg9jbqmqrrxmhn9two6"), 66);
        assert_eq!(line_to_two_digit_number("9eight2six97dkth"), 97);

        assert_eq!(line_to_two_digit_number("sixgjqm64dkvcccvttnts"), 64);
        assert_eq!(line_to_two_digit_number("twofivefourb5four"), 24);
        assert_eq!(line_to_two_digit_number("gfive2"), 52);
        assert_eq!(line_to_two_digit_number("two18twocsxffivetwo4"), 24);
        assert_eq!(line_to_two_digit_number("rmchfml6four6twofive"), 65);
        assert_eq!(line_to_two_digit_number("278eight"), 28);
        assert_eq!(line_to_two_digit_number("six5758jjqpgnvlztwolkcvxtjphd4"), 64);
    }
}