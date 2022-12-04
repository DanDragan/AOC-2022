use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(lines) = read_lines("day4.txt".to_string()) {
        let mut num_overlaps = 0;
        for line in lines {
            let s_line = line.unwrap();

            let v = s_line.split(',').filter_map(|c| { 
                c.split_once('-')
                .and_then(|(l, r)| r.parse().ok().map(|r| (l, r)))
            }).collect::<Vec<(&str, i32)>>();

            let a = v[0].0.parse::<i32>().unwrap();
            let b = v[0].1;
            let c = v[1].0.parse::<i32>().unwrap();
            let d = v[1].1;

            if (a >= c && b <= d) || (a <= c && b >= d) {
                num_overlaps += 1;
            }
        }
        println!("{}", num_overlaps);
    }
}

fn read_lines(filename : String) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}