fn main() {
    let score: u32 = include_str!("../resources/input_2.test")
    .split("\n")
    .map(|line| 
        return get_result(line)
    )
    .sum();

    println!("Final score: {}", score);
}
fn get_result(round: &str) -> u32{
    
    let plays: Vec<String> = round.split(" ").map(|s| s.to_string()).collect();

    let enemy_play = &plays[0];
    let user_play = &plays[1];

    let mut score: u32 = 0;

    //Lose
    if user_play == "X" {
        //Rock
        if enemy_play == "A" {
            score += 3;
        }
        //Paper
        else if enemy_play == "B" {
            score += 1;
        }
        //Scissors
        else if enemy_play == "C" {
            score += 2;
        }
    }
    //Draw
    else if user_play == "Y" {
        score += 3;

        //Rock
        if enemy_play == "A" {
            score += 1;
        }
        //Paper
        else if enemy_play == "B" {
            score += 2;
        }
        //Scissors
        else if enemy_play == "C" {
            score += 3;
        }
    }
    //Win
    else if user_play == "Z" {
        score += 6;
        //Rock
        if enemy_play == "A" {
            score += 2;
        }
        //Paper
        else if enemy_play == "B" {
            score += 3;
        }
        //Scissors
        else if enemy_play == "C" {
            score += 1;
        }
    }
    
    return score;
}

// fn get_result(round: &str) -> u32{
    
//     let plays: Vec<String> = round.split(" ").map(|s| s.to_string()).collect();

//     let enemy_play = &plays[0];
//     let user_play = &plays[1];

//     let mut score: u32 = 0;

//     //Rock
//     if user_play == "X" {
//         score += 1;

//         //Rock
//         if enemy_play == "A" {
//             score += 3;
//         }
//         //Scissors
//         else if enemy_play == "C" {
//             score += 6;
//         }
//     }
//     //Paper
//     else if user_play == "Y" {
//         score += 2;
//         //Paper
//         if enemy_play == "B" {
//             score += 3;
//         }
//         //Rock
//         else if enemy_play == "A" {
//             score += 6;
//         }
//     }
//     //Scissors
//     else if user_play == "Z" {
//         score += 3;
//         //Scissors
//         if enemy_play == "C" {
//             score += 3;
//         }
//         //Paper
//         else if enemy_play == "B" {
//             score += 6;
//         }
//     }
    
//     return score;
// }