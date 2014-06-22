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

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_str()) }
    };

    if matches.opt_present("h") {
        println!("{}", usage("Produce a sorted-by-frequency list of lines from input.", opts));
        return;
    }

    let mut lines: HashMap<String,int> = HashMap::new();

    for line in io::stdin().lines() {
        lines.insert_or_update_with(line.unwrap(), 1, |_k, v| *v = *v + 1);
    }

    let mut sorted_lines: Vec<(int, String)> = Vec::new();

    for (line, count) in lines.iter() {
        sorted_lines.push((*count, line.clone()));
    }

    sorted_lines.sort_by(|a, b| b.cmp(a));

    for &(ref count, ref line) in sorted_lines.iter() {
        print!("{} {}", count, line);
    }
}
