use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part1: {:?}", part1(input));
    println!("Part2: {:?}", part2(input));
}

fn part1(input: &str) -> u32 {
    let chars_both_compartments = input
        .lines()
        .map(|rucksack| {
            let mut chars = rucksack.chars().enumerate().collect::<Vec<(usize, char)>>();
            chars.sort_by(compare_char_then_index);
            (chars, rucksack.len())
        })
        .map(|(chars, len)| {
            chars
                .into_iter()
                .tuple_windows()
                .find_map(move |(a, b)| check_for_duplicate_items(a, b, len))
        });
    let priorities = chars_both_compartments
        .map(Option::unwrap)
        .map(convert_char_to_priority);
    priorities.sum()
}

fn compare_char_then_index(a: &(usize, char), b: &(usize, char)) -> std::cmp::Ordering {
    if a.1 != b.1 {
        a.1.cmp(&b.1)
    } else {
        a.0.cmp(&b.0)
    }
}

fn convert_char_to_priority(char: char) -> u32 {
    if char.is_ascii_lowercase() {
        (char as u32) - 96
    } else {
        (char as u32) - 64 + 26
    }
}

fn check_for_duplicate_items(a: (usize, char), b: (usize, char), len: usize) -> Option<char> {
    if a.1 != b.1 {
        None
    } else if (a.0 > (len / 2) - 1 && a.0 < len && b.0 < (len / 2))
        || (b.0 > (len / 2) - 1 && b.0 < len && a.0 < (len / 2))
    {
        Some(a.1)
    } else {
        None
    }
}

fn part2(input: &str) -> u32 {
    let badges = input
        .lines()
        .map(|x| x.to_owned())
        .tuples()
        .map(|(a, b, c)| {
            a.chars()
                .find(|x| b.contains(*x) && c.contains(*x))
                .unwrap()
        });

    // {
    //     let a_length = a.len();
    //     a.push_str(&b);
    //     a.push_str(&c);
    //     (a, a_length, b.len(), c.len())
    // })
    // .map(|(string, len1, len2, len3)| {
    //     let chars_with_indeces = string
    //         .char_indices()
    //         .sorted_by(compare_char_then_index)
    //         .collect::<Vec<(usize, char)>>();
    //     (chars_with_indeces, len1, len2, len3)
    // })
    // .map(|(chars_with_indeces, len1, len2, len3)| {
    //     let char_groups = chars_with_indeces.into_iter().group_by(|a| a.1);
    //     char_groups
    //         .into_iter()
    //         .filter(|(key, group)| {
    //             let group = group.;

    //             group.any(|(index, _)| index < len1)
    //                 && group.any(|(index, _)| index < len2 && index >= len1)
    //                 && group.any(|(index, _)| index < len3 && index >= len2)
    //         }).map(|(key,_)| key).next().unwrap()
    // });
    let priorities = badges.map(convert_char_to_priority);
    priorities.sum()
}
