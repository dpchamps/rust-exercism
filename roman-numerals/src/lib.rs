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
        (49, "IL"),
        (50, "L"),
        (90, "XC"),
        (99, "IC"),
        (100, "C"),
        (400, "CD"),
        (499, "ID"),
        (500, "D"),
        (900, "CM"),
        (999, "IM"),
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

        if let Some((divisor, numeral)) = divisors.last() {
            if *divisor > number {
                return Roman::calculate_numeral(number, &divisors[0..divisors.len() - 1]);
            }

            return format!(
                "{}{}",
                numeral,
                Roman::calculate_numeral(number - *divisor, divisors)
            );
        }

        unreachable!()
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
