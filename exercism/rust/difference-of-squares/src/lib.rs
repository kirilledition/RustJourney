pub fn square_of_sum(n: u32) -> u32 {
    // todo!("square of sum of 1...{n}")
    // (1..=n).sum::<u32>().pow(2)
    (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // todo!("sum of squares of 1...{n}")
    // (1..=n).map(|x| x.pow(2)).sum()
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n_: u32) -> u32 {
    // todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    // square_of_sum(n) - sum_of_squares(n)
    let n = n_ as f32;
    let first_term = n * (n + 1.) / 2.;
    let second_term = (2. * n + 1.) / 3.;

    let result = first_term * (first_term - second_term);
    result as u32
}

// Sum (1, 2, 3) ^ 2 - (1 ^ 2 + 2 ^ 2 + 3 ^ 2)

// n * (n + 1) * 0.5 * n * (n + 1) * 0.5

// [n * (n + 1) * (2n + 1) * 1 / 6] - [n * (n + 1) * n * (n + 1) / 4]

// n * (n + 1) / 2 {[(2n + 1) * 1 / 3] - [n * (n + 1) / 2]}
