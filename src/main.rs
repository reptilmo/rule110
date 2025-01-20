//! Kostya Leshenko -- CS423 Winter 2025 -- Homework #2
//!
//! Simple [Rule 110](https://en.wikipedia.org/wiki/Rule_110) implementation.


/// Print a row of eight booleans using '*' for true
/// and '.' for false, include new line character
/// at the end.
///
/// * `row` - Reference to an array of eight booleans.
///
/// # Examples
///
/// ```
/// let row = [true; 8];
/// print_row(&row); // Prints ******** to stdout.
///
/// let row2 = [false; 8];
/// print_row(&row2) // Prints ........ to stdout.
/// ```
fn print_row(row: &[bool; 8]) {
    let mut s: String = Default::default();
    for b in row {
        match b {
            true => s += "*",
            false => s += ".",
        }
    }
    println!("{}", s);
}

/// Transforms an array of eight booleans according to Rule 110. 
/// [Rule 110](https://en.wikipedia.org/wiki/Rule_110)
///
/// Returns an array of eight booleans set according to Rule 110 rules.
///
/// * `row` - Reference to a boolean array with eight elements.
///
/// #Examples
/// ```
/// let mut row = [true, false, true, false, false, true, false, false];
/// row = rule110(&row);
/// assert_eq!([true, true, true, false, true, true, false, true], row);
/// ```
fn rule110(row: &[bool; 8]) -> [bool; 8] {
    let mut out = [false; 8];

    for i in 0..=7 {
        let tri = (row[(i + 7) % 8], row[i], row[(i + 1) % 8]);
        match tri {
            (true, true, true) => out[i] = false,    // 111 → 0
            (true, true, false) => out[i] = true,    // 110 → 1
            (true, false, true) => out[i] = true,    // 101 → 1
            (true, false, false) => out[i] = false,  // 100 → 0
            (false, true, true) => out[i] = true,    // 011 → 1
            (false, true, false) => out[i] = true,   // 010 → 1
            (false, false, true) => out[i] = true,   // 001 → 1
            (false, false, false) => out[i] = false, // 000 → 0
        }
    }
    out
}

#[test]
fn test_rule110() {
    let mut row = [true, false, true, false, false, true, false, false];
    row = rule110(&row);
    assert_eq!([true, true, true, false, true, true, false, true], row);

    row = rule110(&[true, true, true, true, true, true, true, true]);
    assert_eq!([false, false, false, false, false, false, false, false], row); 
}

/// Main function for 'rule110' program.
/// 
/// rule110 is able to take one command line
/// parameter for the starting row. This parameter
/// must be provided inside single or double quotes
/// so the shell does not attempt to intepret it
/// as special characters.
/// 
/// # Examples
/// ```
/// $ ./rule110 "..**..*."
/// ..**..*.
/// .***.**.
/// **.****.
/// ****..**
/// ...*.**.
/// ..*****.
/// .**...*.
/// ***..**.
/// *.*.****
/// *****...
/// $
/// ```
fn main() {
    let mut row = [true, false, true, false, false, true, false, false];
    let arg = std::env::args().nth(1);
    if arg.is_some() {
        let s = arg.unwrap();
        for (i, val) in row.iter_mut().enumerate() {
            let c = s.chars().nth(i);
            match c {
                Some(c) => match c {
                    '*' => *val = true,
                    '.' => *val = false,
                    _ => {
                        eprintln!("Unexpected character '{}'!", c);
                        // I don't think this is a good way to return
                        // from main, since the destructors won't be called.
                        // I don't know a better way yet, and I want the
                        // calling process to recieve the error code.
                        std::process::exit(1);
                    }
                },
                None => {
                    eprintln!("Uexpected argument '{}'!", s);
                    std::process::exit(1);
                }
            }
        }
    }

    // Print the rows.
    print_row(&row);
    for _ in 1..=9 {
        row = rule110(&row);
        print_row(&row);
    }
}
