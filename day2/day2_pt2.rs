use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let a: &str = "A";
    let b: &str = "B";
    let c: &str = "C";
    let x: &str = "X";
    let y: &str = "Y";
    let z: &str = "Z";

    let mut score = 0;

    if let Ok(lines) = read_lines("day2.txt".to_string()) {
        for line in lines {
            let s_line = line.unwrap();
            let v_line: Vec<&str> = s_line.split(" ").collect();
            if v_line[0].eq(a) && v_line[1].eq(x) {
                // 0 points for lose, 3 for choosing scissors
                score += 3;
            }
            else if v_line[0].eq(a) && v_line[1].eq(y) {
                // 3 points for draw, 1 for choosing rock
                score += 4;
            }
            else if v_line[0].eq(a) && v_line[1].eq(z) {
                // 6 points for win, 2 for choosing paper
                score += 8;
            }
            else if v_line[0].eq(b) && v_line[1].eq(x) {
                // 0 points for lose, 1 for choosing rock
                score += 1;
            }
            else if v_line[0].eq(b) && v_line[1].eq(y) {
                // 3 points for draw, 2 for choosing paper
                score += 5;
            }
            else if v_line[0].eq(b) && v_line[1].eq(z) {
                // 6 points for win, 3 for choosing scissors
                score += 9;
            }
            else if v_line[0].eq(c) && v_line[1].eq(x) {
                // 0 points for lose, 2 for choosing paper
                score += 2;
            }
            else if v_line[0].eq(c) && v_line[1].eq(y) {
                // 3 points for draw, 3 for choosing scissors
                score += 6;
            }
            else if v_line[0].eq(c) && v_line[1].eq(z) {
                // 6 points for win, 1 for choosing rock
                score += 7;
            }
        }
    }

    println!("{}", score);
}

fn read_lines(filename : String) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}