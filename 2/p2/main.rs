use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

struct Command {
    action: String,
    value: i64
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if (args.len()) > 1 {
        // Do processsing
        let filename = &args[1];
        let input = File::open(filename)?;
        let buffered = BufReader::new(input);

        let mut vec = Vec::new();
        for line in buffered.lines() {
            let line = line.unwrap();
            let mut parts = line.split_whitespace();
            vec.push(Command {
                action: parts.next().unwrap().to_string(), 
                value: parts.next().unwrap().parse().unwrap()
            });
        }
        let mut horizontal_position = 0;
        let mut depth = 0;
        let mut aim = 0;
        for i in 0..vec.len() {
            println!("{} {} {}", i, vec[i].action, vec[i].value);
            if vec[i].action == "forward" {
                horizontal_position = horizontal_position + vec[i].value;
                depth = depth + aim * vec[i].value
            } else if vec[i].action == "up" {
                aim = aim - vec[i].value;
            } else if vec[i].action == "down" {
                aim = aim + vec[i].value;
            }
            println!("{} {} {}", i, horizontal_position, depth);
        }
        let answer: i64 = horizontal_position * depth;
        println!("Answer: {}", answer);
    } else {
        println!("Usage: ./main <inputfile>")
    }


    Ok(())
}
