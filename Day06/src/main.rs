fn main() {
    let input = include_str!("../input.txt");
    let signals = input.char_indices().collect::<Vec<(usize, char)>>();
    
    let amount_processed_part1 = find_start_marker(&signals, 4);
    let amount_processed_part2 = find_start_marker(&signals, 14);

    println!("Part1: {:?}", amount_processed_part1);
    println!("Part2: {:?}", amount_processed_part2);
}

fn find_start_marker(signals: &Vec<(usize, char)>, window_size: usize) -> usize {
    signals
        .windows(window_size)
        .find_map(|x| {
            if x.iter()
                .all(|(_, char)| x.iter().filter(|(_, char2)| char == char2).count() == 1)
            {
                Some(x.last().unwrap().0)
            } else {
                None
            }
        })
        .unwrap()
        + 1
}
