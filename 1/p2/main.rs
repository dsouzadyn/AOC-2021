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
    let mut a: i32 = vec[0];
    let mut b: i32 = vec[1];
    let mut c: i32 = vec[2];
    let mut prev_sum = a + b + c;
    for i in 1..vec.len() - 2 {
        a = vec[i];
        b = vec[i+1];
        c = vec[i+2];
        let current_sum = a + b + c;
        if current_sum > prev_sum {
            total = total + 1;
        }
        prev_sum = current_sum;
    }
    println!("{}", total);

    Ok(())
}
