pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let number_of_rows = minefield.len();

    let number_of_columns = match minefield.iter().next() {
        Some(x) => x.len(),
        _ => 0,
    };

    if number_of_rows == 0 {
        return vec![];
    } else if number_of_columns == 0 {
        return vec![String::new()];
    }

    Vec::<String>::new()
}
