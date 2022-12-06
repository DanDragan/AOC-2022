use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let mut input: BufReader<File> = BufReader::new(File::open("day6.txt").expect("didn't work"));
    let mut str = String::new();
    input.read_to_string(&mut str).expect("cannot read string");

    for i in 0..str.len()-14 {
        let mut arr: [u8; 256] = [0; 256];
        let mut flag = true;

        for j in i..i+14 {
            let x: usize = str.as_bytes()[j] as usize;
            if arr[x] == 1 {
                flag = false;
            }
            arr[x] = 1;
        }

        if flag {
            println!("{}", i + 14);
            break;
        }
    }
}