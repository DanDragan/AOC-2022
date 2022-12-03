use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(lines) = read_lines("day3.txt".to_string()) {
        let mut sum_prio = 0;

        for line in lines {
            let s_line = line.unwrap();
            let s_length = s_line.len();

            let mut array: [i32; 52] = [0; 52];

            for i in 0..s_length / 2 {
                let b: u8 = s_line.as_bytes()[i];
                if b >= 97 && b <= 122 {
                    let index:usize = (b - 97).into();
                    array[index] = 1;
                }
                else if b >= 65 && b <= 90 {
                    let index:usize = (b - 39).into();
                    array[index] = 1;
                }
            }

            for i in s_length / 2..s_length {
                let b: u8 = s_line.as_bytes()[i];
                if b >= 97 && b <= 122 {
                    let index:usize = (b - 97).into();
                    if array[index] == 1 {
                        sum_prio += index + 1;
                        break;
                    }
                }
                else if b >= 65 && b <= 90 {
                    let index:usize = (b - 39).into();
                    if array[index] == 1 {
                        sum_prio += index + 1;
                        break;
                    }
                }
            }
        }
        println!("{}", sum_prio);
    }
}

fn read_lines(filename : String) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}