struct Monkey {
    pub index: u32,
    pub operation: Vec<String>,
    pub items: Vec<usize>,
    pub predicate: usize,
    pub true_action: usize,
    pub false_action: usize,
    pub inspect: usize
}

impl Monkey {
    fn new(index: u32, operation: Vec<String>, items: Vec<usize>, predicate: usize, true_action: usize, false_action: usize, inspect: usize) -> Monkey {
        return Monkey{ index: index, operation: operation, items: items, predicate: predicate, true_action: true_action, false_action:false_action, inspect: inspect };
    }   
}

fn main() {
    let input = include_str!("../resources/input_11.prod").split("\n\n");
    let mut monkeys: Vec<Monkey> = vec![];

    for data in input {
        
        let lines = data.split("\n").collect::<Vec<&str>>();
        
        let index = lines[0].chars().collect::<Vec<char>>()[7].to_digit(10).unwrap();

        let (_, l) = lines[1].split_at(18);
        let temp = l.replace(",", "");
        let items = 
            temp.split_whitespace()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|item| item.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let (_, op) = lines[2].split_at(19);
        let operation = 
            op.split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|operation| operation.to_string())
            .collect::<Vec<String>>();
        
        let (_, predicate) = lines[3].split_at(21);

        let (_, true_action) = lines[4].split_at(29);

        let (_, false_action) = lines[5].split_at(30);


        let monkey = Monkey::new(
            index, 
            operation, 
            items, 
            predicate.parse::<usize>().unwrap(),
            true_action.parse::<usize>().unwrap(), 
            false_action.parse::<usize>().unwrap(),
            0
        );

        monkeys.push(monkey);
    }

    let lcm: usize = monkeys.iter().map(|monkey| monkey.predicate).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let true_action = monkey.true_action;
            let false_action = monkey.false_action;
            let mut items: Vec<(usize, usize)> = vec![];

            for item in &monkey.items {

                let mut new = 0;
                let mut value = 0;

                let v = &monkey.operation[2];

                if v == "old" {
                    value = *item;
                } else {
                    value = v.parse::<usize>().unwrap();
                }

                //println!("{} {} {}", value, monkey.operation[1], item);

                match monkey.operation[1].as_ref() {
                    "*" => new = item * value,
                    "+" => new = item + value,
                    _ => unreachable!("Wrong operation!")
                }

                //new /= 3;

                new = new % lcm; 

                if new % monkey.predicate == 0 {
                    items.push((true_action, new));
                } else {
                    items.push((false_action, new));
                }

                monkey.inspect += 1;
            }
            monkey.items.clear();

            for item in items {
                monkeys[item.0].items.push(item.1);
            }
        }
    }

    let mut inspects = monkeys.iter().map(|monkey| monkey.inspect).collect::<Vec<_>>();

    inspects.sort();

    for item in inspects {
        println!("Inspects: {}", item);
       
        println!("");
    }
}