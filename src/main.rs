extern crate clap;

pub mod lib;
use lib::{calculate_simple, calculate_variable};

use clap::{Arg, App};

fn main() {
    let matches = App::new("set_in_range")
        .version("1.0.0")
        .author("Igor Sowinski <igor@sowinski.blue>")
        .about("constrain a numerical value between a minimum and maximum")
        .arg(Arg::with_name("minimum")
             .required(true)
             .takes_value(true)
             .help("minimum value"))
        .arg(Arg::with_name("maximum")
             .required(true)
             .takes_value(true)
             .help("maximum value"))
        .arg(Arg::with_name("value")
             .required(true)
             .takes_value(true)
             .help("value to set"))
        .arg(Arg::with_name("current_value")
             .required(false)
             .takes_value(true)
             .help("current value (for increments)"))
        .get_matches();

    let maximum = matches.value_of("maximum").and_then(|v| v.trim().parse::<isize>().ok()).unwrap();
    let minimum = matches.value_of("minimum").and_then(|v| v.trim().parse::<isize>().ok()).unwrap();
    let value = matches.value_of("value").unwrap().trim();

    let final_value = match matches.value_of("current_value") {
        Some(current_value) => {
            calculate_variable(minimum, maximum, current_value.trim().parse::<isize>().unwrap(), value)
        },
        None => calculate_simple(minimum, maximum, value.parse::<isize>().unwrap())
    };

    println!("{}", final_value);
}
