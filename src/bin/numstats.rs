use std::error::Error;
use std::fmt;
use std::io;
use std::io::BufRead;

/// For less verbose possibilities, see:
///
///     https://stackoverflow.com/questions/42584368/how-do-you-define-custom-error-types-in-rust
///
/// but it appears that manual implementation of not just description() but also the Display
/// trait appears to be what we have without external crate support.
#[derive(Debug)]
struct InputError {
    desc: String,
}

impl Error for InputError {
    fn description(&self) -> &str {
        return &self.desc;
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

fn main() {
    if let Err(e) = numstats() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn numstats() -> Result<(), Box<Error>> {
    let stdin = io::stdin();
    let mut ns: Vec<f64> = read_floats(&mut stdin.lock())?;

    if ns.contains(&std::f64::NAN) {
        return Err(Box::new(InputError {
            desc: "NaN not supported".to_owned(),
        }));
    }
    sort_floats_by(&mut ns);

    let count = ns.len();

    let sum = ns.iter().fold(0.0, |a, &b| a + b);

    println!("count: {}", count);
    println!("sum:   {}", sum);

    if ns.is_empty() {
        println!("(empty set of numbers, remaining stats not available)");
    } else {
        println!("avg:   {}", sum / count as f64);
        println!("max:   {}", ns[count - 1]);
        println!("p999:  {}", ns[ns.len() * 999 / 1000]);
        println!("p99:   {}", ns[ns.len() * 99 / 100]);
        println!("p90:   {}", ns[ns.len() * 90 / 100]);
        println!("p50:   {}", ns[ns.len() * 50 / 100]);
        println!("min:   {}", ns[0]);
    }

    Ok(())
}

/// Sort a vector under the assumption that all elements in the vector, upon comparison, will
/// result in an order despite the fact the the type does not guarantee it.
///
/// Panics if a comparison fails by not returning an order.
///
/// Per #rust-beginners convo, there may not be a nice way of turning the case into an error.
/// The awkwardness of this is not unknown, e.g. here's an implementation of sort_float():
///
///    https://github.com/notriddle/quickersort#floating-point
///
/// For our purposes here though, I care about education and not performance :P
///
/// Further, unless this function is made specific to floats, it seems impossible to efficiently
/// perform the validation step. We can deduce and hope we're right, that the only special cases
/// that would cause a problem is that of NAN (see https://doc.rust-lang.org/std/f64/ for others).
/// The caller might know that it's a float and do an O(n) validation step. Validating upon a
/// generic T: PartialOrd would require comparing every element with every other element - more
/// expensie than the sort itself is supposed to be!
///
/// As a result, we make this specific to f64 and expect the caller to look for std::f64::NAN
/// prior to calling us.
///
/// Implementing this for both types of float w/o duplication can be done with some generalizations
/// over float types:
///
///    https://github.com/notriddle/quickersort/blob/daf2f70acce76142a56935c6c67e8f6b1c25d313/src/float.rs#L70
fn sort_floats_by(v: &mut Vec<f64>) {
    v.sort_by(|a, b| match a.partial_cmp(b) {
        Some(order) => order,
        None => panic!("sort failed due to partial order; expected an order between all elements"),
    });
}

fn read_floats(input: &mut io::BufRead) -> Result<Vec<f64>, Box<Error>> {
    let ns: Result<Vec<f64>, Box<Error>> = input
        .lines()
        .map(|line| -> Result<f64, Box<Error>> { Ok(line?.parse::<f64>()?) })
        .collect();

    ns
}
