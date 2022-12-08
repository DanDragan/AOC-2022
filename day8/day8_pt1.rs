
use std::fs::read_to_string;

fn main() {
    let input_file_lines: Vec<String> = read_to_string("day8.txt")
        .expect("Could not read input file!")
        .split('\n')
        .map(|line| String::from(line))
        .collect();

    let input_data_length = input_file_lines[0].len(); // Trees in top and bottom rows
    let input_data_width = input_file_lines.len(); // Ignore the four corner trees already  

    let mut visible_trees = (input_data_length*2) + ((input_data_width*2)-4);

    let mut input_data_vector: Vec<Vec<u8>> = Vec::new();

    for i in 0..input_data_width {
        input_data_vector.push(Vec::<u8>::new());
        for j in 0..input_data_length {
            let x = (input_file_lines[i].as_bytes()[j] as u8) - 48;
            input_data_vector[i].push(x);
        }
    }

    for i in 1..input_data_width - 1 {
        for j in 1..input_data_length -1 {
            let mut visible_l = true;
            let mut visible_r = true;
            let mut visible_u = true;
            let mut visible_d = true;

            for k in 0..=j-1 {
                if input_data_vector[i][j] <= input_data_vector[i][k] {
                    visible_l = false;
                }
            }

            for k in j+1..input_data_length {
                if input_data_vector[i][j] <= input_data_vector[i][k] {
                    visible_r = false;
                }
            }

            for k in 0..=i - 1 {
                if input_data_vector[i][j] <= input_data_vector[k][j] {
                    visible_u = false;
                }
            }

            for k in i+1..input_data_width {
                if input_data_vector[i][j] <= input_data_vector[k][j] {
                    visible_d = false;
                }
            }

            if visible_l || visible_r || visible_u || visible_d {
                visible_trees += 1;
            }
        }
    }

    println!("{}", visible_trees);
}