fn main() {
    let input = include_str!("../input.txt");
    let forest = input
        .lines()
        .map(|trees| {
            trees
                .chars()
                .filter_map(|x| x.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let visible_trees = part1(&forest);
    let highest_score = part2(&forest);

    println!("Part1: {:?}", visible_trees);
    println!("Part2: {:?}", highest_score)
}

fn part1(forest: &Vec<Vec<u32>>) -> usize {
    let mut visible_trees = 0usize;
    for y in 0..forest.len() {
        for x in 0..forest[y].len() {
            let current_tree_height = forest[y][x];
            if forest[y]
                .iter()
                .enumerate()
                .filter(|(index, _)| index < &x)
                .all(|(_, height)| height < &current_tree_height)
                || forest[y]
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| index > &x)
                    .all(|(_, height)| height < &current_tree_height)
                || forest
                    .iter()
                    .map(|row| row[x])
                    .enumerate()
                    .filter(|(index, _)| index < &y)
                    .all(|(_, height)| height < current_tree_height)
                || forest
                    .iter()
                    .map(|row| row[x])
                    .enumerate()
                    .filter(|(index, _)| index > &y)
                    .all(|(_, height)| height < current_tree_height)
            {
                visible_trees += 1;
            };
        }
    }
    visible_trees
}


fn part2(forest: &Vec<Vec<u32>>) -> usize {
    let mut scenic_scores = Vec::new();
    for y in 0..forest.len() {
        for x in 0..forest[y].len() {
            let current_tree_height = forest[y][x];
            let mut score_left = forest[y]
                .iter()
                .enumerate()
                .filter(|(index, _)| index < &x)
                .rev()
                .take_while(|(_, &height)| height < current_tree_height)
                .count();
            if score_left != x { score_left += 1;}
            
            let mut score_right = forest[y]
                .iter()
                .enumerate()
                .filter(|(index, _)| index > &x)
                .take_while(|(_, &height)| height < current_tree_height)
                .count();
            if score_right != forest[y].len() - (x+1) { score_right +=1}
            let mut score_up = forest
                .iter()
                .map(|row| row[x])
                .enumerate()
                .filter(|(index, _)| index < &y)
                .rev()
                .take_while(|(_, height)| height < &current_tree_height)
                .count();
            if score_up != y {
                score_up += 1
            }
            let mut score_down = forest
                .iter()
                .map(|row| row[x])
                .enumerate()
                .filter(|(index, _)| index > &y)
                .take_while(|(_, height)| height < &current_tree_height)
                .count();
            if score_down != forest.len() - (y+1) {score_down += 1;}
            scenic_scores.push(score_down * score_left * score_right * score_up);
        }
    }
    scenic_scores.into_iter().max().unwrap()
}
