extern crate clap;

use clap::{Arg, App};
use std::cmp;
use std::process::Command;

enum OffsetType {
    Percentage,
    Absolute
}

enum Offset {
    Plus(OffsetType, isize),
    Minus(OffsetType, isize),
    Invalid
}

impl Offset {
    fn new(value: &str) -> Offset {
        let count = value.chars().count();

        let percentage_sign = value.chars().nth(count - 2).unwrap();
        let offset_type = match percentage_sign {
            '%' => OffsetType::Percentage,
            _ => OffsetType::Absolute
        };

        let num = value.chars()
            .take_while(|v| !['%', '-', '+'].contains(v))
            .collect::<String>()
            .parse::<isize>()
            .unwrap();

        let sign = value.chars().last().unwrap();
        match sign {
            '+' => return Offset::Plus(offset_type, num),
            '-' => return Offset::Minus(offset_type, num),
            _ => return Offset::Invalid
        }
    }
}

fn calculate_simple(minimum: isize, maximum: isize, value: isize) -> isize {
    cmp::min(cmp::max(minimum, value), maximum)
}

fn calculate_variable(minimum: isize, maximum: isize, current_value: isize, offset: &str) -> isize {
    let result = match Offset::new(offset) {
        Offset::Plus(offset_type, v) => match offset_type {
            OffsetType::Absolute => (current_value + v),
            OffsetType::Percentage => (current_value as f32 * (1.0 + 0.01 * v as f32)) as isize
        },
        Offset::Minus(offset_type, v) => match offset_type {
            OffsetType::Absolute => current_value - v,
            OffsetType::Percentage => (current_value as f32 * (1.0 - 0.01 * v as f32)) as isize
        },
        Offset::Invalid => panic!("Incorrect increment value")
    };

    cmp::min(cmp::max(minimum, result), maximum)
}

fn run_command(command: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn();
}

fn main() {
    let matches = App::new("set_in_range")
        .version("1.0.0")
        .author("Igor Sowinski <igor@sowinski.blue>")
        .about("set a system or other value in range")
        .arg(Arg::with_name("command")
             .required(true)
             .takes_value(true)
             .help("command to run. Use $VAL as the placeholder"))
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

    let command = matches.value_of("command").unwrap().trim();
    let maximum = matches.value_of("maximum").and_then(|v| v.trim().parse::<isize>().ok()).unwrap();
    let minimum = matches.value_of("minimum").and_then(|v| v.trim().parse::<isize>().ok()).unwrap();
    let value = matches.value_of("value").unwrap().trim();

    let final_value = match matches.value_of("current_value") {
        Some(current_value) => {
            calculate_variable(minimum, maximum, current_value.trim().parse::<isize>().unwrap(), value)
        },
        None => calculate_simple(minimum, maximum, value.parse::<isize>().unwrap())
    };

    run_command(&command.replace("$VAL", &final_value.to_string()))
}
