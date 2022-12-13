fn main() {

    let (crates, m) = 
        include_str!("../resources/input_5.prod")
        .split_once("\n\n").unwrap();

    let mut crane = create_crane(crates);
    let moves = parse_moves(m.to_string());

    // Part One
    // for mv in moves {

    //     if mv.len() <= 0 {
    //         break;
    //     }

    //     let number = mv[0];
    //     let from = mv[1] - 1;
    //     let to = mv[2] - 1;

    //     let mut index = 0;

    //     while index < number {
    //         if crane[from].len() <= 0 {
    //             break;
            
    //         }
    //         let x = crane[from].pop();

    //         crane[to].push(x.unwrap());
            
    //         index += 1;
    //     }
    // }

    for mv in moves {

        if mv.len() <= 0 {
            break;
        }

        let number = mv[0];
        let from = mv[1] - 1;
        let to = mv[2] - 1;

        let mut index = 0;

        let mut values: Vec<char> = Vec::new();

        while index < number {
            if crane[from].len() <= 0 {
                break;
            }

            let x = crane[from].pop();
            values.push(x.unwrap());
            index += 1;
        }
        values.reverse();
        
        for item in values {
            crane[to].push(item);
        }
    }

    let mut index = 0;
    while index < crane.len() {
        let value = crane[index].pop().unwrap();
        print!("{}", value);
        index += 1;
    }
}

fn create_crane(crates: &str) -> Vec<Vec<char>>{

    let lines = crates
        .lines()
        .rev();

    let first_line: String = lines.clone().take(1).collect::<String>().split_whitespace().collect();
    let mut crane: Vec<Vec<char>> = vec![Vec::new(); first_line.len()];

    for line in lines.skip(1) {

        let data = line.chars().collect::<Vec<char>>();        
        let mut index = 0;
        let mut letter_index = 1;
        
        while letter_index < data.len() - 1 {
            if data[letter_index] != ' '  {
                crane[index].push(data[letter_index]);
            }
            index += 1;
            letter_index += 4;
        }
    }

    return crane;
}

fn parse_moves(moves: String) -> Vec<Vec<usize>> {
    return moves
        .replace("move", "")
        .replace("from", "")
        .replace("to", "")
        .split("\n")
        .map(|line| get_moves(line))
        .collect::<Vec<Vec<usize>>>();
}

fn get_moves(line: &str) -> Vec<usize> {
    let mut moves = Vec::new();

    for item in line.split(" ") {
        if item != " " {
            let parsed_value = item.parse::<usize>();
            if parsed_value.is_ok() {
                moves.push(parsed_value.unwrap());
            }
        }
    }

    return moves;
}