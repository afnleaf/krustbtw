#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display}; 
use rand::prelude::*;


#[derive(Debug,PartialEq)]
enum FileState {
    Open,
    Closed,
}

    #[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}


fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}


impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
    
    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading."));
        }

        let mut tmp = self.data.clone();
        let read_length = tmp.len();
    
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    if one_in(10) {
        let err_msg = String::from("Permission denied.");
        return Err(err_msg);
    }
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    if one_in(10) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let f3_data: Vec<u8> = vec! [
        114, 117, 115, 116, 33
    ];
    let mut f3 = File::new_with_data("3.txt", &f3_data);

    let mut buffer: Vec<u8> = vec![];

    if f3.read(&mut buffer).is_err() {
        println!("Error checking is working.");
    }

    f3 = open(f3).unwrap();
    let f3_length = f3.read(&mut buffer).unwrap();
    f3 = close(f3).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f3);
    println!("{} is {} bytes long", &f3.name, f3_length);
    println!("{}", text);
}
