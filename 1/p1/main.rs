use std::fs::File;
use std::io::{BufReader, BufRead, Error};


fn main() -> Result<(), Error>{
    let filename = "input";
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);

    let mut vec = Vec::new();
    for line in buffered.lines() {
        let num: i32 = line?.trim().parse().expect("Wanted a number");
        vec.push(num);
    }
    let mut total = 0;
    for i in 1..vec.len() {
        let a = vec.get(i);
        let b = vec.get(i-1);
        if a > b {
            total = total + 1;
        }
    }
    println!("{}", total);

    Ok(())
}
