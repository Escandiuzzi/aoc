fn main() {
    let input = include_str!("../resources/input_10.prod");

    let mut value = 1;
    let mut cycles = 0;
    let mut content: String = "".to_string();
    
    for line in input.lines() {
        let trimmed_input = line.replace(" ", "");
        let (command, v) = trimmed_input.split_at(4);

        if command == "noop" {
            evaluate_cycle(&mut cycles, &mut value,&mut content);
        }

        if command == "addx" {
            evaluate_cycle(&mut cycles, &mut value,&mut content);
            evaluate_cycle(&mut cycles, &mut value,&mut content);
            value += v.parse::<i32>().unwrap();
        }
    }

    //println!("Result: {} - {}", result, cycles);
}

fn evaluate_cycle(cycle: &mut i32, value: &mut i32, line: &mut String) {
    if *cycle == *value || *cycle == (*value - 1) || *cycle == (*value + 1) {
        *line += "#";
    } else {
        *line += ".";
    }

    *cycle += 1;
    
    if *cycle % 40 == 0 {
        println!("{}", line);
        *line = "".to_string();
        *value += 40;
    }
}