#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contains<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|x| x == first_list)
}

pub fn sublist<T: PartialEq + std::fmt::Debug>(
    _first_list: &[T],
    _second_list: &[T],
) -> Comparison {
    let sublist = contains(_first_list, _second_list);
    let superlist = contains(_second_list, _first_list);

    match (superlist, sublist) {
        (true, true) => Comparison::Equal,
        (false, false) => Comparison::Unequal,
        (true, false) => Comparison::Superlist,
        (false, true) => Comparison::Sublist,
    }
}
