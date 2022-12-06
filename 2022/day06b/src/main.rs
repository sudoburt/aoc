use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let initial = &input[..14];
    let rest = &input[14..];

    let mut rb: Vec<char> = initial.chars().collect();
    let mut idx = 14;

    for char in rest.chars() {
        rb[idx % 14] = char;
        idx += 1;

        if rb.iter().unique().count() == rb.len() {
            break;
        }
    }

    println!("The amount of characters needed to process is {}", idx);

}
