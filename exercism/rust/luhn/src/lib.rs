/// Check a Luhn checksum.

pub fn is_valid(code: &str) -> bool {
    let code_string = code.replace(" ", "");

    if code_string.len() < 2 {
        return false;
    };

    (match code_string
        .chars()
        .rev()
        .enumerate()
        .map(|(index, digit_candidate)| {
            let mut digit = match digit_candidate.to_digit(10) {
                Some(x) => x,
                None => return Err("Not a digit"),
            };

            if index % 2 != 0 {
                digit *= 2;
            };

            if digit > 9 {
                digit -= 9;
            };

            Ok(digit)
        })
        .sum::<Result<u32, _>>()
    {
        Ok(sum) => sum,
        Err(_) => return false,
    }) % 10
        == 0
}
