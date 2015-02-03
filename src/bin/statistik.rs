use std::cmp;
use std::old_io;
use std::num::from_str_radix;

fn main() {
    let mut ns: Vec<f64> = old_io::stdin()
        .lock()
        .lines()
        .map(|l| from_str_radix::<f64>(l.unwrap().as_slice().trim(), 10))
        .filter(|o| match *o {
            Some(_) => true,
            None => false,
        })
        .map(|n| match n {
            Some(x) => x,
            None => panic!("borkage")
        })
        .collect();

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
