extern crate getopts;
use std::os;
use std::io;
use std::collections::HashMap;

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

    let mut line_counts: HashMap<String,int> = HashMap::new();

    for line_or_fail in io::stdin().lines() {
        // XXX(scode): There is probably a much better way?
        let line = line_or_fail.unwrap();
        let existed = {
            let existing_count = line_counts.get_mut(&line);
            match existing_count {
                None => { false },
                Some(count) => { *count = *count + 1; true},
            }
        };

        if !existed {
            line_counts.insert(line, 1);
        }
    }

    let mut sorted_lines: Vec<(int, String)> = Vec::new();

    for (line, count) in line_counts.iter() {
        sorted_lines.push((*count, line.clone()));
    }

    sorted_lines.sort_by(|a, b| b.cmp(a));

    for &(ref count, ref line) in sorted_lines.iter() {
        print!("{} {}", count, line);
    }
}
