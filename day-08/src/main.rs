use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Deref,
};

enum Instruction {
    Right,
    Left,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => unreachable!(),
        }
    }
}

struct Instructions {
    instructions: Vec<Instruction>,
}

impl From<&str> for Instructions {
    fn from(value: &str) -> Self {
        let instructions = value.chars().map(|c| Instruction::from(c)).collect();
        Instructions { instructions }
    }
}

impl Deref for Instructions {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.instructions
    }
}

struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl From<&'static str> for Node<'static> {
    fn from(value: &'static str) -> Self {
        let split: Vec<&str> = value.split(" ").collect();
        let name = split[0];
        let left = &split[2][1..4];
        let right = &split[3][0..3];
        Node { name, left, right }
    }
}

struct Nodes<'a> {
    nodes: BTreeMap<&'a str, Node<'a>>,
}

impl<'a> Deref for Nodes<'a> {
    type Target = BTreeMap<&'a str, Node<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

struct NavigationMap<'a> {
    instructions: Instructions,
    nodes: Nodes<'a>,
}

impl<'a> NavigationMap<'a> {
    fn count_steps(&self) -> usize {
        let mut current_node_name = "AAA";
        let mut steps = 0;
        let mut instructions = self.instructions.iter().cycle();
        loop {
            if current_node_name == "ZZZ" {
                break;
            }
            let current_node = self.nodes.get(current_node_name).expect("missing node");
            current_node_name = match instructions.next() {
                Some(Instruction::Left) => current_node.left,
                Some(Instruction::Right) => current_node.right,
                None => unreachable!(),
            };
            steps += 1;
        }
        steps
    }

    fn count_steps2(&self, start: &str) -> usize {
        let mut current_node_name = start;
        let mut steps = 0;
        let mut instructions = self.instructions.iter().cycle();
        loop {
            if current_node_name.ends_with("Z") {
                break;
            }
            let current_node = self.nodes.get(current_node_name).expect("missing node");
            current_node_name = match instructions.next() {
                Some(Instruction::Left) => current_node.left,
                Some(Instruction::Right) => current_node.right,
                None => unreachable!(),
            };
            steps += 1;
        }
        steps
    }

    fn count_steps_for_ghosts(&self) -> usize {
        let initial_node_names: Vec<&str> = self
            .nodes
            .keys()
            .filter(|name| name.ends_with("A"))
            .cloned()
            .collect();
        let steps: Vec<usize> = initial_node_names
            .iter()
            .map(|name| self.count_steps2(name))
            .collect();
        let mut lcm = 1;
        for step in steps {
            lcm = num::integer::lcm(lcm, step);
        }
        lcm
    }
}

impl From<&'static str> for NavigationMap<'static> {
    fn from(value: &'static str) -> Self {
        let mut lines = value.lines();
        let instructions = Instructions::from(lines.next().expect("invalid instructions line"));
        lines.next();

        let mut nodes = BTreeMap::new();
        lines.map(|line| Node::from(line)).for_each(|node| {
            nodes.insert(node.name, node);
        });
        let nodes = Nodes { nodes };
        NavigationMap {
            instructions,
            nodes,
        }
    }
}

fn main() {
    let input = include_str!("../../input/day-08");
    let navigation_map = NavigationMap::from(input);
    let steps = navigation_map.count_steps();
    println!("Steps: {}", steps);

    let ghosts_steps = navigation_map.count_steps_for_ghosts();
    println!("Steps for ghosts: {}", ghosts_steps);
}

#[cfg(test)]
mod tests {
    use crate::NavigationMap;

    #[test]
    fn steps() {
        let input = include_str!("../../input/day-08-test");
        let navigation_map = NavigationMap::from(input);
        let steps = navigation_map.count_steps();
        assert_eq!(steps, 2);
    }

    #[test]
    fn steps2() {
        let input = include_str!("../../input/day-08-test2");
        let navigation_map = NavigationMap::from(input);
        let steps = navigation_map.count_steps();
        assert_eq!(steps, 6);
    }

    #[test]
    fn steps3() {
        let input = include_str!("../../input/day-08-test3");
        let navigation_map = NavigationMap::from(input);
        let steps = navigation_map.count_steps_for_ghosts();
        assert_eq!(steps, 6);
    }
}
