use std::{collections::{VecDeque, HashMap}, fmt::Debug};

use itertools::Itertools;

struct Monkey{
    pub starting_items: VecDeque<usize>,
    pub operation: Box<dyn Fn(usize) -> usize>,
    pub test: Box<dyn Fn(usize) -> bool>,
    pub divided_by: usize,
    pub true_index: usize,
    pub false_index: usize,
    pub inspected_items: usize,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey").field("starting_items", &self.starting_items).field("divided_by", &self.divided_by).field("true_index", &self.true_index).field("false_index", &self.false_index).field("inspected_items", &self.inspected_items).finish()
    }
}

impl Monkey{
    pub fn turn(&mut self) -> HashMap<usize, VecDeque<usize>>{
        let mut thrown_items : HashMap<usize, VecDeque<usize>> = HashMap::new();
        for _ in 0..self.starting_items.len(){
            let item = self.starting_items.pop_front().unwrap();
            let during_inspection = (self.operation)(item);
            let after_inspection = during_inspection / 3;
            self.inspected_items +=1;
            let index = if (self.test)(after_inspection) { self.true_index } else { self.false_index};
            thrown_items.entry(index).or_insert(VecDeque::new()).push_back(after_inspection);
        };
        thrown_items
    }

    pub fn turn_part2(&mut self, keep_dividable_by: usize) -> HashMap<usize, VecDeque<usize>>{
        let mut thrown_items : HashMap<usize, VecDeque<usize>> = HashMap::new();
        for _ in 0..self.starting_items.len(){
            let item = self.starting_items.pop_front().unwrap();
            let during_inspection = (self.operation)(item);
            self.inspected_items +=1;
            let index = if (self.test)(during_inspection) { self.true_index } else { self.false_index};
            let reduced = during_inspection % keep_dividable_by;
            thrown_items.entry(index).or_insert(VecDeque::new()).push_back(reduced);
        };
        thrown_items
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut monkeys = parse_monkeys(input);
    // println!("{:#?}", monkeys);
    // for _ in 0..20 {
    //     for i in 0..monkeys.len(){
    //         let thrown_items = monkeys[i].turn();
    //         for (index, mut items) in thrown_items {
    //             monkeys[index].starting_items.append(&mut items);
    //         }
    //     }
    // }
    // let monkey_business = monkeys.iter().sorted_by(|a,b| Ord::cmp(&a.inspected_items, &b.inspected_items)).rev().take(2).fold(1, |acc, v| acc*v.inspected_items);
    // println!("monkey business Part1: {}", monkey_business);

    let keep_dividable_by = monkeys.iter().fold(1, |acc, monkey| acc * monkey.divided_by);
    for _ in 0..10000 {
        for i in 0..monkeys.len(){
            let thrown_items = monkeys[i].turn_part2(keep_dividable_by);
            for (index, mut items) in thrown_items {
                monkeys[index].starting_items.append(&mut items);
            }
        }
    }
    let asdf = monkeys.iter().sorted_by(|a,b| Ord::cmp(&a.inspected_items, &b.inspected_items)).rev().collect::<Vec<_>>();
    println!("{:?}", asdf);
    let monkey_business = monkeys.iter().sorted_by(|a,b| Ord::cmp(&a.inspected_items, &b.inspected_items)).rev().take(2).fold(1, |acc, v| acc*v.inspected_items);
    println!("monkey business Part2: {}", monkey_business);
}

fn parse_monkeys(input: &str) -> Vec<Monkey>{
    input.split("\n\n").map(|monkey| {
        let mut monkey_lines = monkey.lines().skip(1).peekable();
        Monkey {
            starting_items : monkey_lines.next().unwrap().split(": ").skip(1).next().unwrap().split(", ").map(|item| item.parse().unwrap()).collect(),
            operation: parse_operation(monkey_lines.next().unwrap().split(": ").skip(1).next().unwrap()),
            test: parse_test(monkey_lines.peek().unwrap().split_whitespace().last().unwrap()),
            divided_by: monkey_lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap(),
            true_index: monkey_lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap(),
            false_index: monkey_lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap(),
            inspected_items: 0
        }
    }).collect()
}

fn parse_operation(operation: &str) -> Box<dyn Fn(usize) -> usize> {
    let (operation, value) = operation.split_whitespace().skip(3).collect_tuple().unwrap();
    let operation_by_value = if let Ok(number) = value.parse::<isize>() {number} else{ -1};
    match operation {
        "*" => if operation_by_value == -1 { Box::new(|x| x*x)} else {Box::new(move |x| x * usize::try_from(operation_by_value).unwrap())},
        "+" => if operation_by_value == -1 { Box::new(|x| x+x)} else {Box::new(move |x| x + usize::try_from(operation_by_value).unwrap())},
        _ => panic!()
    }
}

fn parse_test(test: &str) -> Box<dyn Fn(usize) -> bool> {
    let divided_by : usize = test.parse().unwrap();
    Box::new(move |x| x % divided_by == 0)
}