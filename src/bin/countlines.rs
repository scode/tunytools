extern crate getopts;
use getopts::Options;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env;
use std::io;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Show help.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        println!(
            "{}",
            opts.usage("Produce a sorted-by-frequency list of lines from input.")
        );
        return;
    }

    let mut line_counts: HashMap<String, isize> = HashMap::new();
    let stdin = io::stdin();
    for line_or_fail in stdin.lock().lines() {
        let entry = line_counts.entry(line_or_fail.unwrap());
        match entry {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
            Entry::Vacant(entry) => {
                entry.insert(1);
            }
        }
    }

    let mut sorted_lines: Vec<(isize, String)> = Vec::new();

    for (line, count) in line_counts.iter() {
        sorted_lines.push((*count, line.clone()));
    }

    sorted_lines.sort_by(|a, b| b.cmp(a));

    for &(ref count, ref line) in sorted_lines.iter() {
        print!("{} {}", count, line);
    }
}
