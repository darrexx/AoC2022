use std::collections::HashMap;

use itertools::Itertools;

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(str: &str) -> Self {
        match str {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!(),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let movements = input
        .lines()
        .map(|line| line.split_whitespace().collect_tuple().unwrap())
        .map(|(direction, count)| (Direction::from(direction), count.parse::<usize>().unwrap()))
        .collect::<Vec<(_, _)>>();

    let part1_visited = part1(movements.clone());
    let part2_visited = part2(movements);

    println!("Part1: {:?}", part1_visited);
    println!("Part2: {:?}", part2_visited);
}

fn part1(movements: Vec<(Direction, usize)>) -> usize {
    let mut visited: HashMap<(isize, isize), bool> = HashMap::new();
    let start_point = (0, 0);
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert(start_point, true);

    for (direction, count) in movements {
        for _ in 0..count {
            match direction {
                Direction::Up => head.1 += 1,
                Direction::Down => head.1 -= 1,
                Direction::Left => head.0 -= 1,
                Direction::Right => head.0 += 1,
            }
            tail = adjust_tail(tail, &head, &direction);
            visited.insert(tail, true);
        }
    }
    visited.len()
}

fn part2(movements: Vec<(Direction, usize)>) -> usize {
    let mut visited: HashMap<(isize, isize), bool> = HashMap::new();
    let start_point = (0, 0);
    let mut knots: Vec<(isize, isize)> = (0..10).map(|_| (0, 0)).collect();
    visited.insert(start_point, true);

    for (direction, count) in movements {
        for _ in 0..count {
            match direction {
                Direction::Up => knots[0].1 += 1,
                Direction::Down => knots[0].1 -= 1,
                Direction::Left => knots[0].0 -= 1,
                Direction::Right => knots[0].0 += 1,
            }
            let mut updated_tails = Vec::new();
            for (head, tail) in knots.clone().into_iter().tuple_windows() {
                if updated_tails.is_empty() {
                    updated_tails.push(adjust_tail(tail, &head, &direction))
                } else {
                    updated_tails.push(adjust_tail(
                        tail,
                        updated_tails.last().unwrap(),
                        &get_direction_for_two_knots(updated_tails.last().unwrap(), &tail),
                    ))
                }
            }
            for (index, tail) in updated_tails.into_iter().enumerate() {
                knots[index + 1] = tail;
            }
            visited.insert(knots.last().unwrap().clone(), true);
        }
    }

    visited.len()
}

fn adjust_tail(
    mut tail: (isize, isize),
    head: &(isize, isize),
    direction: &Direction,
) -> (isize, isize) {
    if (tail.0 - head.0).abs() <= 1 && (tail.1 - head.1).abs() <= 1 {
        tail
    } else {
        match direction {
            Direction::Up => {
                if tail.0 != head.0 {
                    if tail.0 > head.0 {
                        tail.0 -= 1;
                    } else {
                        tail.0 += 1;
                    }
                }
                tail.1 += 1;
            }
            Direction::Down => {
                if tail.0 != head.0 {
                    if tail.0 > head.0 {
                        tail.0 -= 1;
                    } else {
                        tail.0 += 1;
                    }
                }
                tail.1 -= 1;
            }
            Direction::Left => {
                if tail.1 != head.1 {
                    if tail.1 > head.1 {
                        tail.1 -= 1;
                    } else {
                        tail.1 += 1;
                    }
                }
                tail.0 -= 1;
            }
            Direction::Right => {
                if tail.1 != head.1 {
                    if tail.1 > head.1 {
                        tail.1 -= 1;
                    } else {
                        tail.1 += 1;
                    }
                }
                tail.0 += 1;
            }
        }
        tail
    }
}

fn get_direction_for_two_knots(head: &(isize, isize), tail: &(isize, isize)) -> Direction {
    if head.0 < tail.0 {
        Direction::Left
    } else if head.0 > tail.0 {
        Direction::Right
    } else if head.1 > tail.1 {
        Direction::Up
    } else {
        Direction::Down
    }
}