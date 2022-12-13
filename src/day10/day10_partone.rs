fn main() {
    let input = include_str!("../resources/input_10.prod");

    let mut value = 1;
    let mut cycles = 1;
    let mut result = 0;
    
    for line in input.lines() {
        let trimmed_input = line.replace(" ", "");
        let (command, v) = trimmed_input.split_at(4);

        if command == "noop" {
            cycles += 1;
        }

        if command == "addx" {
            cycles += 1;
            evaluate_cycle(&cycles, &value, &mut result);

            cycles += 1;
            value += v.parse::<i32>().unwrap();
        }

        evaluate_cycle(&cycles, &value, &mut result);
    }

    println!("Result: {} - {}", result, cycles);
}

fn evaluate_cycle(cycle: &i32, value: &i32, result: &mut i32) {
    
    if *cycle == 20 || *cycle == 60 || *cycle == 100 || *cycle == 140 || *cycle == 180 || *cycle == 220 {
        let signal = *cycle * value;
        *result += signal;
        println!("{} - {} = {}", cycle, signal, result);
    }
}