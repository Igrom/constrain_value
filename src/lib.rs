use std::cmp;

pub enum OffsetType {
    Percentage,
    Absolute
}

pub enum Offset {
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

pub fn calculate_simple(minimum: isize, maximum: isize, value: isize) -> isize {
    cmp::min(cmp::max(minimum, value), maximum)
}

pub fn calculate_variable(minimum: isize, maximum: isize, current_value: isize, offset: &str) -> isize {
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
