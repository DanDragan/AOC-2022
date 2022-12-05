use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    if let Ok(lines) = read_lines("day5.txt".to_string()) {

        let mut cols = std::iter::repeat(vec![]).take(9).collect::<Vec<_>>();
        let mut r_cols = std::iter::repeat(vec![]).take(9).collect::<Vec<_>>();
        let mut nums = Vec::new();
        let mut froms = Vec::new();
        let mut tos = Vec::new();

        for line in lines {
            let s_line = line.unwrap();

            if s_line.contains('[') {
                for (i, c) in s_line.chars().enumerate() {
                    if c.is_alphabetic() {
                        cols[(i-1)/4].push(c);
                    }
                }
            }
            else if s_line.contains("move") {
                let v_line: Vec<&str> = s_line.split(" ").collect();

                let mut x = 0;

                for elem in v_line {
                    if parsable::<usize>(elem) {
                        if x == 0 {
                            nums.push(elem.parse::<usize>().unwrap());
                        }
                        else if x == 1 {
                            froms.push(elem.parse::<usize>().unwrap());
                        }
                        else if x == 2 {
                            tos.push(elem.parse::<usize>().unwrap());
                        }

                        x += 1;
                    }
                }
            }
        }

        for i in 0..cols.len() {
            while !cols[i].is_empty() {
                let a = cols[i].pop().unwrap();
                r_cols[i].push(a);
            }
        }

        for i in 0..nums.len() {
            let arr = r_cols[froms[i]-1].as_slice()[r_cols[froms[i]-1].len()-nums[i]..].to_vec();
            for j in 0..nums[i] { 
                r_cols[froms[i]-1].pop();
                r_cols[tos[i]-1].push(arr[j]);
            }
        }

        for c in r_cols {
            print!("{}", c[c.len()-1]);
        }
        println!();
    }
}

fn parsable<T: FromStr>(s: &str) -> bool {
    s.parse::<T>().is_ok()
}

fn read_lines(filename : String) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}