struct Directory {
    name: String,
    pub size: u32
}

impl Directory {
    fn new(name: String, size: u32) -> Directory {
        return Directory {name, size};
    }
}

fn main() {
    let data = include_str!("../resources/input_7.prod");
    
    let mut directories: Vec<Directory> = Vec::new();
    let filysystem_space = 70_000_000;
    let required_space = 30_000_000;
    let mut candidates: Vec<u32> = Vec::new();
    
    //let mut total = 0;

    for line in data.lines() {
        if line == "$ ls" || line == " " {
            continue;
        }

        if line.starts_with("dir") {
            continue;
        }
        
        //Part One
        // if line.starts_with("$ cd") {
        //     let dir = &line[5..];

        //     if dir == ".." {
        //         let dir = directories.pop().unwrap();

        //         if dir.size <= 100_000 {
        //             total += dir.size;
        //         }
                
        //         let last = directories.last_mut().unwrap();
        //         last.size += dir.size;

        //     } else {
        //         let dir = Directory::new(dir.clone().to_string(), 0);
        //         directories.push(dir);
        //     }
        // } else {
        //     let (amount, _) = line.split_once(" ").unwrap();
        //     let dir = directories.last_mut().unwrap();
            
        //     dir.size += amount.parse::<u32>().unwrap();
        // }

        if line.starts_with("$ cd") {
            let dir = &line[5..];

            if dir == ".." {
                let dir = directories.pop().unwrap();
                candidates.push(dir.size);
                
                let last = directories.last_mut().unwrap();
                last.size += dir.size;

            } else {
                let dir = Directory::new(dir.clone().to_string(), 0);
                directories.push(dir);
            }
        } else {
            let (amount, _) = line.split_once(" ").unwrap();
            let dir = directories.last_mut().unwrap();
            
            dir.size += amount.parse::<u32>().unwrap();
        }
    }
    let total:u32 = directories.into_iter().map(|dir| dir.size).sum();
    let free_space = filysystem_space - total;

    let result = candidates.into_iter()
                    .filter(|item| item + free_space >= required_space)
                    .min()
                    .unwrap();

    print!("Result: {}", result);
}