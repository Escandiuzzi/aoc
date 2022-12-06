fn main() {

    // First part
    // let value : i32= 
    //     include_str!("../resources/input_4.test")
    //     .lines()
    //     .map(|pair| {
    //         let vec = pair.split(",").collect::<Vec<_>>();
    
    //         let first = 
    //             vec[0].split("-").map(|value| value.parse::<i32>().unwrap()).collect::<Vec<_>>();

    //         let second = 
    //             vec[1].split("-").map(|value| value.parse::<i32>().unwrap()).collect::<Vec<_>>();

    //         if first_contains_second(&first, &second) || second_contains_first(&first, &second) {
    //             return 1;
    //         }
    //         return 0;
    //     })
    // .sum();

    // Second Part
    let value : i32= 
        include_str!("../resources/input_4.test")
        .lines()
        .map(|pair| {
            let vec = pair.split(",").collect::<Vec<_>>();
    
            let first = 
                vec[0].split("-").map(|value| value.parse::<i32>().unwrap()).collect::<Vec<_>>();

            let second = 
                vec[1].split("-").map(|value| value.parse::<i32>().unwrap()).collect::<Vec<_>>();

            if values_overlap(&first, &second) {
                return 1;
            }
            return 0;
        })
    .sum();
    
    println!("Sum result: {}", value);

}

fn values_overlap(first: &Vec<i32>, second: &Vec<i32>) -> bool {
    return (first[0] <= second[0] && first[1] >= second[1]) ||
           (first[0] == second[1] || first[1] == second[0]) ||
           (first[0] < second[0] && first[1] > second[0]) ||
           (first[0] > second[0] && first[0] < second[1]) ||
           (first[0] >= second[0] && first[1] <= second[1]);
}

// fn first_contains_second(first: &Vec<i32>, second: &Vec<i32>) -> bool {
//     return first[0] <= second[0] && first[1] >= second[1];
// }

// fn second_contains_first(first:  &Vec<i32>, second: &Vec<i32>) -> bool {
//     return first[0] >= second[0] && first[1] <= second[1];
// }