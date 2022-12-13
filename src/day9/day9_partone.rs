use std::collections::{HashSet};

fn main() {
    let input = include_str!("../resources/input_9.prod");

    let mut visited:HashSet<(usize, usize)> = HashSet::new();
    let mut t = (1000, 1000);
    let mut last_position: (usize, usize) = (1000, 1000);
    
    visited.insert((1000,1000));

    let mut x:usize = 1000;
    let mut y:usize = 1000;
    for line in input.lines() {
        
        let (direction, steps) = line.split_once(" ").unwrap();   
        for _ in 0..(steps.parse::<usize>().unwrap()) {
            match direction {
                "U" => {
                    y += 1;
                },
                "D" => {
                    y -= 1;
                },
                "L" => {
                    x -= 1;
                },
                "R" => {
                    x += 1;
                },
                _ => println!("Invalid direction {}", direction)
            }

            println!("{}x{}", x, y);
            println!("{}x{}", t.0, t.1);

            let mut x_result = (t.0 as i32) - (x as i32);
            if x_result < 0 {
                x_result *= -1;
            }

            let mut y_result = (t.1 as i32) - (y as i32);
            if y_result < 0 {
                y_result *= -1;
            }

            if x_result >= 11 || y_result >= 11 {
                t = last_position;
                if !visited.contains(&t) {
                    println!("Insert - {} {}", t.0, t.1);
                    visited.insert(t);
                }
            }
            println!(" ");
            last_position = (x, y);
        }
    }

    println!("{}", visited.len());

}