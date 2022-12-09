use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    if let Ok(lines) = read_lines("day9.txt".to_string()) {
        let mut head = Point::default();
        let mut tail = Point::default();

        let mut visited: HashSet<Point> = HashSet::with_capacity(1000);

        for line in lines {
            let s_line = line.unwrap();

            let v: Vec<&str> = s_line.split(' ').collect();

            let steps: u32 = v[1].parse().unwrap();

            for _i in 0..steps {
                match v[0] {
                    "L" => head.x -= 1,
                    "R" => head.x += 1,
                    "U" => head.y += 1,
                    "D" => head.y -= 1,
                    _ => panic!("shall not happen"),
                }

                move_tail(&head, &mut tail);
                visited.insert(tail);
            }
        }
        println!("{}", visited.len());
    }
}

fn move_tail(h: &Point, t: &mut Point) {
    let dx = h.x - t.x;
    let dy = h.y - t.y;
    if (dx.abs() + dy.abs() > 1) && (dx.abs() > 1 || dy.abs() > 1) {
        t.x += dx.signum();
        t.y += dy.signum()
    }
}

fn read_lines(filename : String) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}