#![feature(iter_array_chunks)]
use std::collections::{HashSet};

fn main() {
    // First Part
    // let value:u32 = 
    //     include_str!("../resources/input_3.test")
    //         .lines()
    //         .flat_map(|line| return get_items(line))
    //         .map(|item| {
    //             if item.is_lowercase() {
    //                 return (item as u32) - 96;
    //             }
    //             return (item as u32) - 38;
    //         })
    //         .sum();

    // Seconds Part
    let value: u32 = 
        include_str!("../resources/input_3.prod")
        .lines()
        .array_chunks::<3>()
        .flat_map(|group| {
            let a = group[0].chars().collect::<HashSet<_>>();
            let b = group[1].chars().filter(|c| a.contains(c)).collect::<HashSet<_>>();
            return group[2].chars().filter(|c| b.contains(c)).collect::<HashSet<_>>();
        })
        .map(|badge| {
            if badge.is_lowercase() {
                return (badge as u32) - 96;
            }
            return (badge as u32) - 38;
        })
        .sum();

    println!("Sum of priorities: {}", value);
}

fn get_items(line: &str) -> HashSet<char>{

    let compartment_size = line.len() / 2;
    let(compartment1, compartment2) = line.split_at(compartment_size);

    let compartment1 = compartment1.chars().collect::<HashSet<_>>();

    return
        compartment2
            .chars()
            .filter(|letter| compartment1.contains(letter))
            .collect::<HashSet<_>>();
}