use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
enum Pair {
    Num(u8),
    List(Vec<Pair>),
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Num(b)) => a.cmp(&vec![Self::Num(*b)]),
            (Self::Num(a), Self::List(b)) => vec![Self::Num(*a)].cmp(&b),
            (Self::Num(a), Self::Num(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let pairs = parse();

    let mut res1 = 0;

    for i in 0..pairs.len() {
        if pairs[i][0] < pairs[i][1] {
            res1 += i + 1;
        }
    }

    println!("Part 1: {}", res1);

    let mut arr: Vec<_> = pairs.iter().flatten().collect();

    let divider_1 = parse_pair("[[2]]");
    let divider_2 = parse_pair("[[6]]");

    arr.push(&divider_1);
    arr.push(&divider_2);

    arr.sort_unstable();

    let mut res2 = 1;

    for i in 0..arr.len() {
        if arr[i] == &divider_1 || arr[i] == &divider_2 {
            res2 *= i+1;
        }
    }

    println!("Part 2: {}", res2);
}

fn parse() -> Vec<[Pair; 2]> {
    let file = std::fs::read_to_string("day13.txt").unwrap();

    file
        .split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines();
            let left = lines.next().unwrap();
            let right = lines.next().unwrap();

            [parse_pair(left), parse_pair(right)]
        })
        .collect()
}

fn parse_pair(s: &str) -> Pair {
    let chars: Vec<_> = s.chars().collect();
    let (pair, _) = parse_list(&chars);
    pair
}

fn parse_num(list: &[char]) -> (Pair, &[char]) {
    let mut i = 0;
    while i < list.len() && list[i].is_ascii_digit() {
        i += 1;
    }

    let mut num = 0;
    let mut offset = 1;
    for c in list[..i].iter().rev() {
        num += (*c as u8 - b'0') * offset;
        offset *= 10;
    }

    (Pair::Num(num), &list[i..])
}

fn parse_list(list: &[char]) -> (Pair, &[char]) {
    let mut list = &list[1..];
    let mut pairs = Vec::new();

    loop {
        match list[0] {
            ']' => break,
            ',' => list = &list[1..],
            '[' => {
                let (pair, rest) = parse_list(list);
                pairs.push(pair);
                list = rest;
            }
            _ => {
                let (n, rest) = parse_num(list);
                pairs.push(n);
                list = rest;
            }
        }
    }

    (Pair::List(pairs), &list[1..])
}