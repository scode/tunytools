extern crate getopts;
use std::os;
use std::old_io;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use getopts::getopts;
use getopts::optflag;
use getopts::usage;

fn main() {
    let args: Vec<String> = os::args()
        .iter()
        .map(|x| x.to_string())
        .collect();

    let opts = [
        optflag("h", "help", "Show help.")
    ];

    let matches = match getopts(args.tail(), &opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        println!("{}", usage("Produce a sorted-by-frequency list of lines from input.", &opts));
        return;
    }

    let mut line_counts: HashMap<String,isize> = HashMap::new();

    for line_or_fail in old_io::stdin().lock().lines() {
        let entry = line_counts.entry(line_or_fail.unwrap());
        match entry {
            Entry::Occupied(mut entry) => { *entry.get_mut() += 1; },
            Entry::Vacant(entry) => { entry.insert(1); },
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
