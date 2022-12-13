use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let calories_per_elf = input
        .split("\n\n")
        .map(|x| x
            .split("\n")
            .map(|calories| calories.parse::<usize>().unwrap_or(0))
            .sum())
        .collect::<Vec<usize>>();
    println!("Part1: {:#?}", part1(calories_per_elf.clone()));
    println!("Part1: {:#?}", part2(calories_per_elf));
}

fn part1(calories_list: Vec<usize>) -> usize {
    calories_list.into_iter().max().unwrap()
}

fn part2(calories_list: Vec<usize>) -> usize {
    calories_list.into_iter().sorted().rev().take(3).sum()
}