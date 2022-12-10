use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(lines) = read_lines("day10.txt".to_string()) {
        let mut regx = 1;
        let mut step = 1;
        let mut sig_strength = 0;

        for line in lines {
            let s_line = line.unwrap();

            let v: Vec<&str> = s_line.split(' ').collect();

            if v[0] == "noop" {
                if step == 20 || (step - 20) % 40 == 0 {
                    sig_strength += step * regx;
                }
                step += 1;
            }
            else if v[0] == "addx" {
                for _ in 0..2 {
                    if step == 20 || (step - 20) % 40 == 0 {
                        sig_strength += step * regx;
                    }
                    step += 1;
                }
                regx += v[1].parse::<i32>().unwrap();
            }
        }

        println!("{}", sig_strength);
    }
}

fn read_lines(filename : String) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}