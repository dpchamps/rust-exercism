fn encode_char(count: i32, c: char) -> String {
    let string_count = if count == 0 {
        String::new()
    } else {
        (count + 1).to_string()
    };

    let char_string = if c == '\0' {
        String::new()
    } else {
        c.to_string()
    };

    format!("{}{}", string_count, char_string)
}


pub fn encode(source: &str) -> String {
    let mut last = '\0';
    let mut count = 0;
    let mut result = String::new();

    for c in source.chars() {
        if c == last {
            count = count + 1;
        } else if last != '\0' {
            result.push_str(
                &encode_char(count, last)
            );
            count = 0;
        }

        last = c;
    }

    format!("{}{}", result, encode_char(count, last))
}

pub fn decode(source: &str) -> String {
    let mut num_string = String::from("0");
    let mut result = String::new();

    for c in source.chars() {
        if c.is_digit(10) {
            num_string.push_str(&c.to_string());
            continue;
        }

        let repeats = std::cmp::max(1, num_string.parse::<usize>().unwrap());
        let fragment = std::iter::repeat(c).take(repeats).collect::<String>();

        result.push_str(&fragment);
        num_string = String::from("0");
    }

    result
}
