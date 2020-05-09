#[macro_use]
extern crate lazy_static;

use std::fmt::{Display, Formatter};

lazy_static! {
    static ref base_divisors: Vec<(u32, &'static str)> = vec![
        (1, "I"),
        (4, "IV"),
        (5, "V"),
        (9, "IX"),
        (10, "X"),
        (40, "XL"),
        (50, "L"),
        (90, "XC"),
        (100, "C"),
        (400, "CD"),
        (500, "D"),
        (900, "CM"),
        (1000, "M"),
    ];
}

pub struct Roman {
    numeral: String,
}

impl Roman {
    fn calculate_numeral(number: u32, divisors: &[(u32, &'static str)]) -> String {
        if divisors.len() == 0 || number == 0 {
            return String::new();
        }

        match divisors[divisors.len() - 1] {
            (divisor, numeral) if divisor > number => {
                Roman::calculate_numeral(number, &divisors[0..divisors.len() - 1])
            }
            (divisor, numeral) => format!(
                "{}{}",
                numeral,
                Roman::calculate_numeral(number - divisor, divisors)
            ),
        }
    }
}

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(_f, "{}", self.numeral)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman {
            numeral: Roman::calculate_numeral(num, &base_divisors[..]),
        }
    }
}

// Iterative Version:

// let mut remainder = num;
//
// let numeral = base_divisors.iter().rev().fold(String::from(""), |numeral, &(divisor, base_numeral)|{
//     let mut sub_result = String::from("");
//     loop {
//         if divisor > remainder{
//             break;
//         }
//
//         remainder -= divisor;
//         sub_result = format!("{}{}", sub_result, base_numeral);
//     }
//
//     format!("{}{}", numeral, sub_result)
// });
