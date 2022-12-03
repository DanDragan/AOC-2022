use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(lines) = read_lines("day3.txt".to_string()) {
        let mut sum = 0;

        let mut array = vec![vec![0; 52]; 3];
        let mut r = 0;

        for line in lines {
            let s_line = line.unwrap();
            let s_length = s_line.len();

            for i in 0..s_length {
                let b: u8 = s_line.as_bytes()[i];
                if b >= 97 && b <= 122 {
                    let index:usize = (b - 97).into();
                    array[r][index] = 1;
                }
                else if b >= 65 && b <= 90 {
                    let index:usize = (b - 39).into();
                    array[r][index] = 1;
                }
            }

            r += 1;

            if r == 3 {
                for i in 0..52 {
                    if array[0][i] == 1 && array[0][i] == array[1][i] && array[1][i] == array[2][i] {
                        sum += i + 1;
                    }
                    array[0][i] = 0;
                    array[1][i] = 0;
                    array[2][i] = 0;
                }
                r = 0;
            }
        }
        println!("{}", sum);
    }
}

fn read_lines(filename : String) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}