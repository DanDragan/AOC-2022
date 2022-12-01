use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut max_elf_cals: [i32; 3] = [0, 0, 0];
    let mut crt_elf_cals = 0;
    let mut sum_cals = 0;

    if let Ok(lines) = read_lines("day1.txt".to_string()) {
        for line in lines {
            let s_line = line.unwrap();

            if s_line.is_empty() == false {
                crt_elf_cals += s_line.parse::<i32>().unwrap();
            }
            else
            {
                for i in 0..=2 {
                    if crt_elf_cals > max_elf_cals[i] {
                        for j in (i+1..=2).rev() {
                            max_elf_cals[j] = max_elf_cals[j-1]; 
                        }
                        max_elf_cals[i] = crt_elf_cals;
                        break;
                    }
                }
                crt_elf_cals = 0;
            }
        }
    }

    for i in 0..=2 {
        sum_cals += max_elf_cals[i];
    }

    println!("{}", sum_cals);
}

fn read_lines(filename : String) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}