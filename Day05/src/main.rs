use itertools::Itertools;
#[macro_use]
extern crate scan_fmt;

type Crate = char;

fn main() {
    let input = include_str!("../input.txt");
    let (stacks, procedures) = input.split("\n\n").collect_tuple().unwrap();

    let stacks = create_initial_stacks(stacks);
    let initial_stacks_part2 = stacks.clone();

    let final_stacks_part1 = run_procedures(stacks, procedures, CrateMoverVersion::V9000);
    let final_stacks_part2 =
        run_procedures(initial_stacks_part2, procedures, CrateMoverVersion::V9001);

    println!(
        "Part 1 Top of Stack: {:?}",
        final_stacks_part1
            .into_iter()
            .map(|mut x| x.pop().unwrap())
            .join("")
    );
    println!(
        "Part 2 Top of Stack: {:?}",
        final_stacks_part2
            .into_iter()
            .map(|mut x| x.pop().unwrap())
            .join("")
    );
}

fn create_initial_stacks(input: &str) -> Vec<Vec<Crate>> {
    let mut input = input.lines().rev();

    let mut stacks: Vec<Vec<Crate>> = input
        .next()
        .unwrap()
        .split_whitespace()
        .map(|_| Vec::new())
        .collect();

    for stackline in input {
        stackline
            .char_indices()
            .filter_map(|(index, char)| if index % 4 == 1 { Some(char) } else { None })
            .enumerate()
            .for_each(|(index, char)| {
                if !char.is_whitespace() {
                    stacks[index].push(char)
                }
            });
    }
    stacks
}

fn run_procedures(
    mut stacks: Vec<Vec<Crate>>,
    procedures: &str,
    version: CrateMoverVersion,
) -> Vec<Vec<Crate>> {
    procedures
        .lines()
        .map(|procedure| {
            scan_fmt!(procedure, "move {d} from {d} to {d}", usize, usize, usize).unwrap()
        })
        .map(|(amount, source, destination)| (amount, source - 1, destination - 1))
        .for_each(|(amount, source_index, destination_index)| {
            let drain_start = stacks[source_index].len() - amount;
            let drained = stacks[source_index].drain(drain_start..);
            let mut drained = match version {
                CrateMoverVersion::V9000 => drained.rev().collect::<Vec<char>>(),
                CrateMoverVersion::V9001 => drained.collect::<Vec<char>>(),
            };
            stacks[destination_index].append(&mut drained);
        });
    stacks
}

enum CrateMoverVersion {
    V9000,
    V9001,
}
