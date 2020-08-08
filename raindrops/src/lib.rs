pub fn raindrops(n: u32) -> String {
    let result = vec![3, 5, 7].iter().fold(String::new(), |str, factor| {
        if n % factor != 0 {
            return str;
        }

        let noise = match factor {
            3 => "Pling",
            5 => "Plang",
            7 => "Plong",
            _ => unreachable!(),
        };

        format!("{}{}", str, noise)
    });

    if &result == "" {
        n.to_string()
    } else {
        result
    }
}
