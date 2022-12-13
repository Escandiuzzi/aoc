fn main() {
    
    let content = std::fs::read_to_string("./resources/input_1.prod")
        .expect("File not found!");

    let elfs_supply = content.split("\n\n");

    let mut total: Vec<u32> = Vec::new(); 

    for supply in elfs_supply {

        let food_calories = supply.split("\n");
        let mut total_calories = 0;

        for calories in food_calories {

            let value = calories.parse::<u32>();

            if value.is_ok() {
                total_calories += value.unwrap();
            }
        }

        total.push(total_calories);
    }

    total.sort_by(|a,b| b.cmp(a));

    println!("Max value: {}", total[..3].iter().sum::<u32>());

    //{
        // First Part - ThePrimeagen
        // let max = include_str!("../resources/input_1.txt")
        // .split("\n\n")
        // .map(|supply| {
        //         return supply
        //             .split("\n")
        //             .flat_map(str::parse::<usize>)
        //             .sum::<usize>(); 
        //     })
        //     .max();
        
        // println!("Max: {}", max.unwrap());

        // Second Part - ThePrimeagen
        // let mut max = include_str!("../resources/input_1.txt")
        // .split("\n\n")
        // .map(|supply| {
        //         return supply
        //             .lines()
        //             .flat_map(str::parse::<usize>)
        //             .sum::<usize>(); 
        //     })
        //     .collect::<Vec<usize>>();

        // println!("Max: {}", max.into_iter().take(3).sum::<usize>());
    //}
}