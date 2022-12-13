use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use itertools::Itertools;

struct ArenaTree<T> {
    nodes: Vec<Node<T>>,
}

#[derive(Clone, Debug)]
struct Node<T> {
    index: usize,
    parent: Option<usize>,
    children: Vec<usize>,
    data: T,
}

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Clone, Debug)]
struct Directory {
    name: String,
}

#[derive(Clone, Debug)]
enum FileSystemElement {
    File(File),
    Directory(Directory),
}

fn main() {
    let input = include_str!("../input.txt");
    let mut tree = ArenaTree::<FileSystemElement> { nodes: Vec::new() };
    let root_element = Node::<FileSystemElement> {
        index: 0,
        children: Vec::new(),
        data: FileSystemElement::Directory(Directory {
            name: String::from("/"),
        }),
        parent: None,
    };
    let mut current_element = root_element.clone();
    tree.nodes.push(root_element);

    let commands_and_output = input.split("$ ");
    for command_and_output in commands_and_output.skip(1) {
        let mut iterator = command_and_output.lines();
        let command = iterator.next().unwrap().split(" ").collect::<Vec<&str>>();
        match command[0] {
            "ls" => {
                // println!("ls");
                iterator
                    .map(|output_line| output_line.split_whitespace().collect::<Vec<&str>>())
                    .for_each(|output_line| {
                        if let Ok(size) = output_line[0].parse::<usize>() {
                            // println!("{}", size);
                            let file = FileSystemElement::File(File {
                                name: String::from(output_line[1]),
                                size,
                            });
                            let index = tree.nodes.len();
                            let node = Node {
                                index,
                                parent: Some(current_element.index),
                                children: Vec::new(),
                                data: file,
                            };
                            tree.nodes.push(node);
                            tree.nodes[current_element.index].children.push(index);
                            // println!("{:#?}", current_element);
                        } else {
                            let new_directory = FileSystemElement::Directory(Directory {
                                name: String::from(output_line[1]),
                            });
                            let index = tree.nodes.len();
                            let node = Node {
                                index,
                                parent: Some(current_element.index),
                                children: Vec::new(),
                                data: new_directory,
                            };
                            tree.nodes.push(node);
                            tree.nodes[current_element.index].children.push(index);
                        }
                    });
            }
            "cd" => {
                if command[1] == ".." {
                    println!("..{:?}", current_element);
                    current_element = tree.nodes[current_element.parent.unwrap()].clone();
                } else {
                    current_element = tree
                        .nodes
                        .clone()
                        .into_iter()
                        .enumerate()
                        .filter_map(|(index, elem)| if current_element.children.contains(&index) {  Some(elem)} else { None})
                        .find(|x| match &x.data {
                            FileSystemElement::File(_) => false,
                            FileSystemElement::Directory(directory) => directory.name == command[1],
                        })
                        .unwrap();
                    
                        println!("{:?}", current_element);
                }
            }
            _ => panic!("asdf"),
        }
    }
    let directory_sizes = tree
        .nodes
        .iter()
        .filter_map(|x| {
            if let FileSystemElement::Directory(_) = x.data {
                Some((x, calculate_size(x, &tree)))
            } else {
                None
            }
        })
        .collect::<Vec<(_, _)>>();

        println!("{:#?}", directory_sizes);
    let smallest_directory_sizes = directory_sizes
        .iter()
        .filter_map(|(_, size)| if *size <= 100000 { Some(*size) } else { None })
        .collect::<Vec<usize>>();

    println!("{:#?}", smallest_directory_sizes);

    println!("Part1: {}", smallest_directory_sizes.iter().sum::<usize>());
}

fn calculate_sizes(tree: ArenaTree<FileSystemElement>) -> ArenaTree<FileSystemElement> {
    let current_element = &tree.nodes[0];

    tree
}

fn calculate_size(
    directory: &Node<FileSystemElement>,
    tree: &ArenaTree<FileSystemElement>,
) -> usize {
    println!("a");
    let children = tree
        .nodes
        .iter()
        .enumerate()
        .filter_map(|(index, node)| {
            if directory.children.contains(&index) {
                Some(node)
            } else {
                None
            }
        })
        .collect::<Vec<&Node<FileSystemElement>>>();
    println!("{:#?}::::::{:#?}", directory, children);
    let mut sum = 0usize;
    for child in children {
        match &child.data {
            FileSystemElement::File(file) => sum = sum + file.size,
            FileSystemElement::Directory(_) => sum = sum + calculate_size(&child, tree),
        };
        println!("{}", sum);
    }
    sum
}

// fn process_command<'a>(
//     input: Vec<&'a str>,
//     current_element: &'a mut Node<FileSystemElement>,
//     tree: &'a mut ArenaTree<FileSystemElement>,
// ) -> &'a Node<FileSystemElement> {
//     let mut iterator = input.iter();
//     let command = iterator.next().unwrap().split(" ").collect::<Vec<&str>>();
//     match command[0] {
//         "ls" => {
//             iterator
//                 .map(|output_line| output_line.split_whitespace().collect::<Vec<&str>>())
//                 .for_each(|output_line| {
//                     if let Ok(size) = output_line[0].parse::<usize>() {
//                         let file = FileSystemElement::File(File {
//                             name: String::from(output_line[1]),
//                             size,
//                         });
//                         let index = tree.nodes.len();
//                         let node = Node {
//                             index,
//                             parent: Some(current_element.index),
//                             children: Vec::new(),
//                             data: file,
//                         };
//                         tree.nodes.push(node);
//                         current_element.children.push(index);
//                     } else {
//                         let new_directory = FileSystemElement::Directory(Directory {
//                             name: String::from(output_line[1]),
//                             size: 0,
//                         });
//                         let index = tree.nodes.len();
//                         let node = Node {
//                             index,
//                             parent: Some(current_element.index),
//                             children: Vec::new(),
//                             data: new_directory,
//                         };
//                         tree.nodes.push(node);
//                         current_element.children.push(index);
//                     }
//                 });
//             &current_element
//         }
//         "cd" => {
//             if command[1] == ".." {
//                 return &tree.nodes[current_element.parent.unwrap()];
//             }
//             tree.nodes
//                 .iter()
//                 .find(|x| match x.data {
//                     FileSystemElement::File(_) => false,
//                     FileSystemElement::Directory(directory) => directory.name == command[1],
//                 })
//                 .unwrap()
//         }
//         _ => panic!("asdf"),
//     }
// }
