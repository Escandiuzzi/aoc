fn main() {
    let grid = 
        include_str!("../resources/input_8.prod")
            .lines()
            .map(|item| 
                item.chars().collect::<Vec<_>>()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
            )
            .collect::<Vec<Vec<u32>>>();

    let rows = grid.len();
    let columns = grid[0].len();

    // Part One
    // let mut result = (rows * 2) + ((columns * 2) - 4);
    // for x in 1..rows - 1 {          
    //     for y in 1..columns - 1 {
    //         let tree = grid[x][y];

    //         if grid[0..x].iter().all(|item| item[y] < tree) || 
    //            grid[(x + 1)..].iter().all(|item| item[y] < tree) {
    //             result += 1;
    //             continue;
    //         }

    //         if grid[x][0..y].iter().all(|item| item < &tree) || 
    //            grid[x][(y + 1)..].iter().all(|item| item < &tree) {
    //             result += 1;
    //             continue;
    //         }
    //     }
    // }

    // println!("{}", result);

    let mut score = 0;
    for x in 1..(rows - 1) {
        for y in 1..(columns - 1) {
            let tree = grid[x][y];

            // x - 1 -> 0 
            let top = get_score_vertical(&grid, (0..x).rev().collect::<Vec<usize>>(), &tree, y);           
            
            // x + 1 -> rows
            let bottom = get_score_vertical(&grid, ((x + 1)..rows).into_iter().collect::<Vec<usize>>(), &tree, y);
            
            // y - 1 -> 0
            let left = get_score_horizontal(&grid, (0..y).rev().collect::<Vec<usize>>(), &tree, x);
            
            // y + 1 -> columns
            let right = get_score_horizontal(&grid, ((y + 1)..columns).into_iter().collect::<Vec<usize>>(), &tree, x);
          
            let _score = top * bottom * left * right;
            
            // println!("{} - {}x{}", tree, x, y);
            // println!("{} - {} {} {} {}", _score, top, bottom, left, right);
            // println!("");

            if _score > score {
                score = _score;
            }
        }
    }

    println!("{}", score);

}

fn get_score_vertical(grid: &Vec<Vec<u32>>, range: Vec<usize>,tree: &u32, y: usize) -> u32 {
    
    let mut score = 0;
    
    for x in range {
        let neighbor = grid[x][y];
        score += 1;
        if neighbor >= *tree { break; }
    }
    
    return score;
}

fn get_score_horizontal(grid: &Vec<Vec<u32>>, range: Vec<usize>,tree: &u32, x: usize) -> u32 {
    
    let mut score = 0;
    
    for y in range {
        let neighbor = grid[x][y];
        score += 1;
        if neighbor >= *tree { break; }
    }
    
    return score;
}

