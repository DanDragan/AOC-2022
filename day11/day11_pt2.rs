use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(Option<i64>),
    Mul(Option<i64>),
}

#[derive(Debug, Clone)]
struct Monkey {
    worry_lvls: Vec<i64>,
    func: Operation,
    divisibility: i64,
    true_monkey_id: u8,
    false_monkey_id: u8,
    insp: i64,
}

impl Default for Monkey {
    fn default() -> Self {
        Self {
            worry_lvls: vec![],
            func: Operation::Add(None),
            divisibility: 0,
            true_monkey_id: 255,
            false_monkey_id: 255,
            insp: 0,
        }
    }
}

fn main() {
    if let Ok(lines) = read_lines("day11.txt".to_string()) {
        let mut monkeys = vec![];
        let mut monkey_id: usize = 0;

        for line in lines {
            let s_line = line.unwrap();
            let v: Vec<&str> = s_line.split(' ').collect();

            if s_line.contains("Monkey") {
                monkey_id = v[1].strip_suffix(":").unwrap().parse::<usize>().unwrap();
                let monkey = Monkey::default();
                monkeys.push(monkey);
            }
            else if s_line.contains("Starting items:") {
                for i in 4..v.len()-1 {
                    let item = v[i].strip_suffix(",").unwrap().parse::<i64>().unwrap();
                    monkeys[monkey_id].worry_lvls.push(item);
                }

                let item = v[v.len()-1 ].parse::<i64>().unwrap();
                monkeys[monkey_id].worry_lvls.push(item);
            }
            else if s_line.contains("Operation:") {
                let param = if v[7] == "old" {
                    None
                } else {
                    Some(v[7].parse::<i64>().expect("valid number"))
                };

                if v[6] == "+" {
                    monkeys[monkey_id].func = Operation::Add(param);
                }
                else if v[6] == "*" {
                    monkeys[monkey_id].func = Operation::Mul(param);
                }
            }
            else if s_line.contains("Test:") {
                monkeys[monkey_id].divisibility = v[5].parse::<i64>().unwrap();
            }
            else if s_line.contains("If true:") {
                monkeys[monkey_id].true_monkey_id = v[9].parse::<u8>().unwrap();
            }
            else if s_line.contains("If false:") {
                monkeys[monkey_id].false_monkey_id = v[9].parse::<u8>().unwrap();
            }
        }

        let modulos = monkeys
        .iter()
        .map(|monkey| monkey.divisibility)
        .product::<i64>();

        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                while let Some(worry_lvl) = monkeys[i].worry_lvls.pop() {
                    let mut worry_lvl = worry_lvl;
                    match monkeys[i].func {
                        Operation::Mul(None) => worry_lvl *= worry_lvl,
                        Operation::Add(None) => worry_lvl += worry_lvl,
                        Operation::Mul(Some(n)) => worry_lvl *= n,
                        Operation::Add(Some(n)) => worry_lvl += n,
                    }

                    worry_lvl %= modulos;
                    let result = if worry_lvl % monkeys[i].divisibility == 0 {
                        monkeys[i].true_monkey_id as usize
                    } else {
                        monkeys[i].false_monkey_id as usize
                    };
                    monkeys[result].worry_lvls.push(worry_lvl);
                    monkeys[i].insp += 1;
                }
            }
        }

        let mut insp: Vec<i64> = monkeys.iter().map(|monkey| monkey.insp).collect();
        insp.sort_by(|a, b| b.cmp(a));

        println!("{}", insp[0] * insp[1]);
    }
}

fn read_lines(filename : String) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}