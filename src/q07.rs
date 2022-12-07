use crate::parser::*;
use std::io;

const INPUT_FILE: &str = "input/input07.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input07_sample.txt";

#[derive(Debug)]
enum Node {
    Dir(Option<usize>, String, Vec<usize>),
    File(usize, String, u32),
}

impl Node {
    fn size(&self, all_nodes: &Vec<Node>) -> u32 {
        match self {
            Node::File(_, _, size) => *size,
            Node::Dir(_, _, children) => children
                .iter()
                .map(|x| &all_nodes[*x])
                .map(|x| x.size(all_nodes))
                .sum(),
        }
    }
}

struct Input {
    nodes: Vec<Node>,
}

impl TryFrom<&mut FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let mut all_nodes = Vec::<Node>::new();
        all_nodes.push(Node::Dir(None, String::from("/"), Vec::new()));
        let mut current_dir = 0;
        while let Some(line) = lines.next() {
            match line {
                s if s == "$ ls" => {}
                s if s == "$ cd /" => current_dir = 0,
                s if s == "$ cd .." => {
                    current_dir = match all_nodes[current_dir] {
                        Node::Dir(Some(n), _, _) => n,
                        Node::File(n, _, _) => n,
                        Node::Dir(None, _, _) => {
                            return Err(io::Error::new(
                                io::ErrorKind::Other,
                                "Tried to cd .. from root directory",
                            ));
                        }
                    }
                }
                s if s.starts_with("$ cd") => {
                    let dir_name = s.rsplit_once(' ').unwrap().1;
                    current_dir = match &all_nodes[current_dir] {
                        Node::Dir(_, _, children) => *children
                            .iter()
                            .filter(|child| match &all_nodes[**child] {
                                Node::Dir(_, name, _) => name == dir_name,
                                _ => false,
                            })
                            .next()
                            .unwrap(),
                        _ => current_dir,
                    }
                }
                s if s.starts_with("dir") => {
                    let (_, dir_name) = s.split_once(' ').unwrap();
                    let dir_node = Node::Dir(Some(current_dir), String::from(dir_name), Vec::new());
                    add_child(&mut all_nodes, current_dir, dir_node);
                }
                s => {
                    let (size_str, file_name) = s.split_once(' ').unwrap();
                    let size: u32 = size_str.parse().unwrap();
                    let file_node = Node::File(current_dir, String::from(file_name), size);
                    add_child(&mut all_nodes, current_dir, file_node);
                }
            }
        }
        Ok(Input { nodes: all_nodes })
    }
}

pub fn part_1() -> io::Result<u32> {
    let input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_1(input))
}

fn _part_1_sample() -> io::Result<u32> {
    let input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_1(input))
}

fn do_part_1(input: Input) -> u32 {
    let sizes = get_directory_sizes(&input.nodes);
    sizes.iter().filter(|x| **x <= 100_000).sum()
}

pub fn part_2() -> io::Result<u32> {
    let input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_2(input))
}

fn _part_2_sample() -> io::Result<u32> {
    let input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_2(input))
}

fn do_part_2(input: Input) -> u32 {
    const TOTAL_SPACE: u32 = 70_000_000;
    const NEEDED_SPACE: u32 = 30_000_000;
    let free_space = TOTAL_SPACE - input.nodes[0].size(&input.nodes);
    let mut sizes = get_directory_sizes(&input.nodes);
    sizes.sort();
    for size in sizes {
        if size + free_space >= NEEDED_SPACE {
            return size;
        }
    }
    0
}

fn add_child(nodes: &mut Vec<Node>, parent: usize, child: Node) {
    let i = nodes.len();
    match nodes[parent] {
        Node::Dir(_, _, ref mut children) => {
            children.push(i);
            nodes.push(child);
        }
        _ => {}
    }
}

fn add_sizes_from(nodes: &Vec<Node>, node: usize, sizes: &mut Vec<u32>) {
    if let Node::Dir(_, _, ref children) = nodes[node] {
        sizes.push(nodes[node].size(nodes));
        for child in children.iter() {
            add_sizes_from(nodes, *child, sizes);
        }
    }
}

fn get_directory_sizes(nodes: &Vec<Node>) -> Vec<u32> {
    let mut sizes = Vec::<u32>::new();
    add_sizes_from(&nodes, 0, &mut sizes);
    sizes
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(1206825, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(95437, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(9608311, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(24933642, _part_2_sample().unwrap());
    }
}
