use std::collections::HashSet;
use std::cmp;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Rock,
    Sand,
    Air,
}

impl Point {
    fn neighbours(&self) -> [Point; 3] {
        let down = Point {
            x: self.x,
            y: self.y + 1,
        };
        let down_left = Point {
            x: self.x - 1,
            y: self.y + 1,
        };
        let down_right = Point {
            x: self.x + 1,
            y: self.y + 1,
        };

        [down, down_left, down_right]
    }

    fn next_p1(&self, cave: &[Vec<Tile>]) -> Option<Point> {
        self.neighbours()
            .iter()
            .find(|point| cave[point.y as usize][point.x as usize] == Tile::Air).copied()
    }

    fn next_p2(&self, cave: &[Vec<Tile>], floor_y: i32) -> Option<Point> {
        if (self.y + 1) == floor_y {
            return None;
        }
        self.neighbours()
            .iter()
            .find(|p| cave[p.y as usize][p.x as usize] == Tile::Air).copied()
    }
}

fn main() {
    let rock_lines = parse();
    let rocks1:HashSet<Point> = get_rocks(rock_lines);
    let rocks2:HashSet<Point> = rocks1.clone();

    println!("Part 1: {}", part1(rocks1));
    println!("Part 2: {}", part2(rocks2));
}

fn part1(rocks: HashSet<Point>) -> i32 {
    let start = Point { x: 500, y: 0 };
    let max_y = rocks.iter().map(|p| p.y).max().unwrap();
    let width = 1500;

    let mut cave = vec![vec![Tile::Air; width as usize]; (max_y + 2) as usize];
    for pos in rocks {
        cave[pos.y as usize][pos.x as usize] = Tile::Rock;
    }

    for i in 0.. {
        let mut sand = start;
        while let Some(next_coord) = sand.next_p1(&cave) {
            sand = next_coord;
            if sand.y > max_y {
                return i;
            }
        }

        cave[sand.y as usize][sand.x as usize] = Tile::Sand;
    }

    unreachable!();
}

fn part2(rocks: HashSet<Point>) -> i32 {
    let start = Point { x: 500, y: 0 };
    let max_y = rocks.iter().map(|p| p.y).max().unwrap();
    let width = 1500;

    let mut cave = vec![vec![Tile::Air; width as usize]; (max_y + 2) as usize];
    for pos in rocks {
        cave[pos.y as usize][pos.x as usize] = Tile::Rock;
    }

    for i in 0.. {
        let mut sand = start;
        // the sand falls until it can't anymore and next returns None
        while let Some(next_air_coord) = sand.next_p2(&cave, max_y + 2) {
            sand = next_air_coord;
        }
    
        // insert final coord into the cave as sand tile
        cave[sand.y as usize][sand.x as usize] = Tile::Sand;
    
        if sand == start {
            return i + 1;
        }
    }

    unreachable!();
}

fn parse() -> Vec<Vec<Point>> {
    let input = std::fs::read_to_string("day14.txt").unwrap();

    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|points| {
                    let (x, y) = points.split_once(',').unwrap();
                    let x = x.parse().unwrap();
                    let y = y.parse().unwrap();
                    Point { x, y }
                })
                .collect()
        })
        .collect()
}

fn get_rocks(rock_lines: Vec<Vec<Point>>) -> HashSet<Point> {

    let mut hs:HashSet<Point> = HashSet::<Point>::new();

    for rock_line in rock_lines {
        for i in 0..rock_line.len()-1 {
            if rock_line[i].x == rock_line[i+1].x {
                let start_y = cmp::min(rock_line[i].y, rock_line[i+1].y);
                let end_y = cmp::max(rock_line[i].y, rock_line[i+1].y);

                for j in start_y..=end_y {
                    let mut p = Point::default();
                    p.x = rock_line[i].x;
                    p.y = j;
                    hs.insert(p);
                }
            }
            else if rock_line[i].y == rock_line[i+1].y {
                let start_x = cmp::min(rock_line[i].x, rock_line[i+1].x);
                let end_x = cmp::max(rock_line[i].x, rock_line[i+1].x);

                for j in start_x..=end_x {
                    let mut p = Point::default();
                    p.x = j;
                    p.y = rock_line[i].y;
                    hs.insert(p);
                }
            }
        }
    }

    hs
}