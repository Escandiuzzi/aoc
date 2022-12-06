use std::collections::HashSet;

fn main() {
    let chars = include_str!("../resources/input_6.prod")
    .chars()
    .collect::<Vec<char>>();

    let mut index = 0;
    let mut helper = 0;
    let mut count = 0;

    let mut signal: HashSet<char> = HashSet::new();

    // For the first part change to 4
    while count != 14 {
        let current_letter: &char = &chars[helper];
        
        if !signal.contains(current_letter) {
            signal.insert(current_letter.clone());
            count += 1;
            
        } else {
            signal.clear();
            count = 0;
            helper = index;
            index += 1;
        }

        helper += 1;
    }

    for letter in signal {
        print!("{}", letter);
    }
    
    // For the first part change to 4
    println!("\nIndex of signal: {}", index + 14);
}