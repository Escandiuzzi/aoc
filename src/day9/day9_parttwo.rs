use std::collections::{HashSet};

// type Pos = (i64, i64);
fn main() {
    let input = include_str!("../resources/input_9.prod");

    let mut visited:HashSet<(i64, i64)> = HashSet::new();
    let mut nodes = vec![(0, 0); 10];

    visited.insert((0,0));


    for line in input.lines() {
        let (direction, steps) = line.split_once(" ").unwrap();   
        for _ in 0..(steps.parse::<i64>().unwrap()) {
            match direction {
                "U" => {
                    nodes[0].1 += 1;
                },
                "D" => {
                    nodes[0].1 -= 1;
                },
                "L" => {
                    nodes[0].0 -= 1;
                },
                "R" => {
                    nodes[0].0 += 1;
                },
                _ => unreachable!(),
            }

            for i in 1..nodes.len() {
                if let Some(pos) = move_node(&nodes[i], &nodes[i - 1]) {
                    nodes[i] = pos;
                } else {
                    break;
                }
            }
            visited.insert(*nodes.last().unwrap());
            
        }
        println!(" ");
    }

    println!("{}", visited.len());
}

fn move_node(tail: &(i64, i64), head: &(i64, i64)) -> Option<(i64, i64)> {
    let distance_x = tail.0 - head.0;
    let distance_y = tail.1 - head.1;

    if (distance_x == 2 || distance_x == -2) && (distance_y == 2 || distance_y == -2) {
        Some((head.0 + distance_x.clamp(-1, 1), head.1 + distance_y.clamp(-1, 1)))
    } else if distance_x == 2 || distance_x == -2 {
        Some((head.0 + distance_x.clamp(-1, 1), head.1))
    } else if distance_y == 2 || distance_y == -2 {
        Some((head.0, head.1 + distance_y.clamp(-1, 1)))
    } else {
        None
    }
}