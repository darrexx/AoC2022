fn main() {
    let input = include_str!("../input.txt");
    let pairs = input
        .lines()
        .map(|line| line.split(&[',', '-']).collect::<Vec<&str>>())
        .map(|strings| {
            strings
                .into_iter()
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|assignments| {
            (
                assignments[0]..=assignments[1],
                assignments[2]..=assignments[3],
            )
        })
        .collect();

    println!("Part1: {:#?}", part1(&pairs));
    println!("Part2: {:#?}", part2(&pairs));
}

fn part1(
    pairs: &Vec<(
        std::ops::RangeInclusive<usize>,
        std::ops::RangeInclusive<usize>,
    )>,
) -> usize {
    pairs
        .into_iter()
        .filter(|(range1, range2)| {
            range1.clone().all(|x| range2.contains(&x))
                || range2.clone().all(|x| range1.contains(&x))
        })
        .count()
}

fn part2(
    pairs: &Vec<(
        std::ops::RangeInclusive<usize>,
        std::ops::RangeInclusive<usize>,
    )>,
) -> usize {
    pairs
        .into_iter()
        .filter(|(range1, range2)| {
            range1.clone().any(|x| range2.contains(&x))
                || range2.clone().any(|x| range1.contains(&x))
        })
        .count()
}
