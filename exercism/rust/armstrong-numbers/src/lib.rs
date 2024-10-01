pub fn is_armstrong_number(num: u32) -> bool {
    let number_as_string = num.to_string();
    let power = number_as_string.len() as u32;

    let armsrong_number = number_as_string.chars().fold(0, |accumulator, x| {
        accumulator + x.to_digit(10).unwrap().pow(power)
    });

    armsrong_number == num
}
