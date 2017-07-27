use std::cmp;
use std::io;
use std::io::BufRead;
use std::error::Error;

fn main() {
    let stdin = io::stdin();
    let mut ns: Vec<f64> = match read_floats(&mut stdin.lock()) {
        Ok(ns) => ns,
        Err(e) => panic!("failed to read numbers: {}", e),
    };

    ns.sort_by(|a, b| a.partial_cmp(b).unwrap_or(cmp::Ordering::Equal));

    let count = ns.len();

    let sum = ns.iter().fold(0.0, |a, &b| a + b);

    println!("count: {}", count);
    println!("sum:   {}", sum);

    if ns.is_empty() {
        println!("(empty set of numbers, remaining stats not available)");
    } else {
        println!("avg:   {}", sum/count as f64);
        println!("max:   {}", ns[count - 1]);
        println!("p999:  {}", ns[ns.len() * 999/1000]);
        println!("p99:   {}", ns[ns.len() * 99/100]);
        println!("p90:   {}", ns[ns.len() * 90/100]);
        println!("p50:   {}", ns[ns.len() * 50/100]);
        println!("min:   {}", ns[0]);
    }
}

fn read_floats(input: &mut io::BufRead) -> Result<Vec<f64>, Box<Error>> {
    let mut ns: Result<Vec<f64>, Box<Error>> = input
        .lines()
        .map(|line| -> Result<f64, Box<Error>> {
            Ok(line?.parse::<f64>()?)
        })
        .collect();

    ns
}
