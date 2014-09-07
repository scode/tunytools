//use std::os;
use std::io;
use std::num::from_str_radix;

fn main() {
    let mut ns: Vec<f64> = io::stdin()
        .lines()
        .map(|l| from_str_radix::<f64>(l.unwrap().as_slice().trim(), 10))
        .filter(|o| match *o {
            Some(_) => true,
            None => false,
        })
        .map(|n| match n {
            Some(x) => x,
            None => fail!("borkage")
        })
        .collect();

    ns.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));

    let count = ns.len();

    let mut sum: f64 = 0.0; // no reduce in rust? searched, failed to find
    for n in ns.iter() {
        sum = sum + *n
    }

    println!("count: {}", count);
    println!("sum:   {}", sum);

    if ns.is_empty() {
        println!("(empty set of numbers, remaining stats not available)");
    } else {
        println!("avg:   {}", sum/count as f64);
        println!("max:   {}", ns[count - 1]);
        println!("p999:  {}", ns[ns.len() * 999/1000]);
        println!("p99:  {}", ns[ns.len() * 99/100]);
        println!("p90:  {}", ns[ns.len() * 90/100]);
        println!("p50:  {}", ns[ns.len() * 50/100]);
        println!("min:   {}", ns[0]);
    }
}
