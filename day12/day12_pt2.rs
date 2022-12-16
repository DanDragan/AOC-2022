
use std::fs::read_to_string;
use std::collections::VecDeque;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let input_file_lines: Vec<String> = read_to_string("day12.txt")
        .expect("Could not read input file!")
        .split('\n')
        .map(|line| String::from(line))
        .collect();

    let length = input_file_lines[0].len();
    let width = input_file_lines.len();
    let mut end = Point::default();

    let mut data_vec: Vec<Vec<char>> = Vec::new();
    let mut visited: Vec<Vec<u32>> = Vec::new();
    let mut deque: VecDeque<Point> = VecDeque::new();

    for i in 0..width {
        data_vec.push(Vec::<char>::new());
        visited.push(Vec::<u32>::new());

        for j in 0..length {
            let mut elem : char = input_file_lines[i].as_bytes()[j] as char;

            if elem == 'S' {
                elem = 'a';
            }
            else if elem == 'E' {
                end.x = i as i32;
                end.y = j as i32;
                deque.push_back(end);
                elem = 'z';
            }

            data_vec[i].push(elem);
            visited[i].push(0);
        }
    }

    let mut min = 0x7FFF;

    while !deque.is_empty() {
        let p = deque.pop_front().unwrap();

        let mut n = Point::default();
        let mut found = false;

        // left neighbour
        if p.x - 1 >= 0 && ((data_vec[p.x as usize][p.y as usize] as u8 ) <= (data_vec[(p.x - 1) as usize][p.y as usize] as u8) + 1) && visited[(p.x - 1) as usize][p.y as usize] == 0 {
            n.x = (p.x - 1) as i32;
            n.y = p.y as i32;
            deque.push_back(n);
            visited[n.x as usize][n.y as usize] = visited[p.x as usize][p.y as usize] + 1;
            found  = true;
        }

        // right neighbour
        if p.x + 1 < (width as i32) && ((data_vec[p.x as usize][p.y as usize] as u8) <= (data_vec[(p.x + 1) as usize][p.y as usize] as u8) + 1) && visited[(p.x + 1) as usize][p.y as usize] == 0 {
            n.x = (p.x + 1) as i32;
            n.y = p.y as i32;
            deque.push_back(n);
            visited[n.x as usize][n.y as usize] = visited[p.x as usize][p.y as usize] + 1;
            found = true;
        }

        // up neighbour
        if p.y - 1 >= 0 && ((data_vec[p.x as usize][p.y as usize] as u8) <= (data_vec[p.x as usize][(p.y - 1) as usize] as u8) + 1) && visited[p.x as usize][(p.y - 1) as usize] == 0 {
            n.x = p.x as i32;
            n.y = (p.y - 1) as i32;
            deque.push_back(n);
            visited[n.x as usize][n.y as usize] = visited[p.x as usize][p.y as usize] + 1;
            found = true;
        }

        // down neighbour
        if p.y + 1 < (length as i32) && ((data_vec[p.x as usize][p.y as usize] as u8) <= (data_vec[p.x as usize][(p.y + 1) as usize] as u8) + 1) && visited[p.x as usize][(p.y + 1) as usize] == 0 {
            n.x = p.x as i32;
            n.y = (p.y + 1) as i32;
            deque.push_back(n);
            visited[n.x as usize][n.y as usize] = visited[p.x as usize][p.y as usize] + 1;
            found = true;
        }

        if found && data_vec[n.x as usize][n.y as usize] == 'a' {
            if min > visited[n.x as usize][n.y as usize] {
                min = visited[n.x as usize][n.y as usize];
            }
        }
    }

    println!("{}", min);
}