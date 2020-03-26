
enum SayError {
    GeneralErr(&'static str)

}

type SayResult = Result<String, SayError>;

fn say_scale(n : usize) -> SayResult {
    let quantifier = match n {
        1 => "",
        2 => "thousand",
        3 => "million",
        4 => "billion",
        5 => "trillion",
        6 => "quadrillion",
        7 => "quintillion",
        _ => return Err(SayError::GeneralErr("Number too big :("))
    };

    Ok(quantifier.to_string())
}

fn say_tens(tens: usize) -> SayResult {
    let tens_string = match tens {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => return Err(SayError::GeneralErr("Invalid tens input"))
    };

    Ok(tens_string.to_string())
}

fn say_teens(ones : usize) -> SayResult {
    let teens_string = match ones {
        0 => "ten",
        1 => "eleven",
        2 => "twelve",
        3 => "thirteen",
        4 => "fourteen",
        5 => "fifteen",
        6 => "sixteen",
        7 => "seventeen",
        8 => "eighteen",
        9 => "nineteen",
        _ => return Err(SayError::GeneralErr("Invalid teens input"))
    };

    Ok(teens_string.to_string())
}

fn say_ones(n : usize) -> SayResult {
    let one_string = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => return Err(SayError::GeneralErr("Invalid Ones Digit"))
    };

    return Ok(one_string.to_string());
}

fn say_grouping(group : usize) -> SayResult {
    let hundred = group / 100;
    let tens = (group % 100) / 10;
    let ones = group % 10;

    let mut group_strings = vec![];

    match hundred {
        0 => (),
        _ => group_strings.push(format!("{} hundred", say_ones(hundred)?))
    };

    match tens {
        0 => {
            if ones == 0 && hundred == 0 || ones != 0{
                group_strings.push(say_ones(ones)?)
            }
        },
        1 => group_strings.push(say_teens(ones)?),
        _ => {
            if ones == 0{
                group_strings.push(say_tens(tens)?)
            }else{
                group_strings.push(
                    format!("{}-{}", say_tens(tens)?, say_ones(ones)?)
                )
            }
        }
    };

    Ok(group_strings.join(" "))
}

fn count_groupings(n :&u64) -> usize {
    let mut copy = n.clone() / 10;
    let mut place : usize = 1;
    loop {
        if copy == 0 {
            break;
        }
        copy = copy / 10;
        place += 1;
    }

    return (f64::ceil(place as f64 / 3.0)) as usize;
}

fn get_group(number : u64, position : u32) -> u64{
    let mut result: u64 = 0;

    for i in 0..3 {
        let (digit_position, did_overflow) = u64::overflowing_pow(10, position * 3 + i);
        if did_overflow{
            break;
        }
        let digit = number / digit_position % 10;
        result += digit * u64::pow(10, i);
    }

    result
}

pub fn encode(n: u64) -> String {
    let mut string_group = vec![];
    let n_groupings = count_groupings(&n);

    for i in 0..n_groupings {
        let next_group = get_group(n, i as u32) as usize;

        if next_group == 0 && i+1 < n_groupings {
            continue;
        }

        match say_scale(i+1){
            Ok(scale) if scale != String::from("")=> string_group.push(scale),
            _ => (),
            Err(SayError::GeneralErr(message)) => panic!(message)
        };

        match say_grouping(next_group){
            Ok(word) => string_group.push(word),
            Err(SayError::GeneralErr(message)) => panic!(message)
        };
    }

    string_group.reverse();
    string_group.join(" ")
}
