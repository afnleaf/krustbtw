// Advent of Code 2024: 1
// https://adventofcode.com/2024/day/1
// input.txt

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

const FILENAME: &str = "input.txt";

fn read_file(filename: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    // read file once to get lines and number of lines
    let file = File::open(filename)?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()?;
    let size = lines.len();
    // keep it on the stack
    let mut list1 = Vec::with_capacity(size);
    let mut list2 = Vec::with_capacity(size);
    // iterate through all lines
    for line in lines {
        if line.trim().is_empty() { break };
        // collect numbers separated by whitespace
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();
        // push to two separated lists
        assert!(numbers.len() == 2);
        list1.push(numbers[0]);
        list2.push(numbers[1]);
        //println!("{}\t{}",  numbers[0], numbers[1]);
    }
    // return both lists
    Ok((list1, list2))
}

fn main() -> io::Result<()> {
    let (list1, list2) = read_file(FILENAME)?;
    
    // part 1
    println!("Part 1: ");

    let mut a = list1.clone();
    let mut b = list2.clone();

    a.sort();
    b.sort();

    let mut distance: i32 = 0;
    
    for i in 0..a.len() {
        let c = (a[i] - b[i]).abs();    
        //println!("{} - {} = {}", a[i], b[i], c);
        distance += c;
    }

    println!("{}", distance);

    // part 2
    println!("Part 2: ");
    
    // use hashmap to process right side list2
    let mut map = HashMap::new();
    for &num in list2.iter() {
        if let Some(count) = map.get_mut(&num) {
            *count += 1;
        } else {
            map.insert(num, 1);
        }
    }
    // then access the values in the map
    let mut similarity = 0;
    for &num in list1.iter() {
        let value = map.get(&num).unwrap_or(&0);
        similarity += num * value;
    }
    println!("{}", similarity);
    // pretty fast lookup
    // need to refine this code
    Ok(())
}
