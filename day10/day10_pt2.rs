use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(lines) = read_lines("day10.txt".to_string()) {
        let mut sprite_pos = 0;
        let mut pixel_pos = 0;

        for line in lines {
            let s_line = line.unwrap();

            let v: Vec<&str> = s_line.split(' ').collect();

            if v[0] == "noop" {
                if pixel_pos >= sprite_pos && pixel_pos <= sprite_pos + 2 {
                    print!("{}", "#")
                }
                else {
                    print!("{}", ".");
                }
                pixel_pos += 1;

                if pixel_pos % 40 == 0 {
                    pixel_pos = 0;
                    println!();
                }
            }
            else if v[0] == "addx" {
                for _ in 0..2 {
                    if pixel_pos >= sprite_pos && pixel_pos <= sprite_pos + 2 {
                        print!("{}", "#")
                    }
                    else {
                        print!("{}", ".");
                    }

                    pixel_pos += 1;

                    if pixel_pos % 40 == 0 {
                        pixel_pos = 0;
                        println!();
                    }
                }
                sprite_pos += v[1].parse::<i32>().unwrap();
            }
        }
    }
}

fn read_lines(filename : String) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}