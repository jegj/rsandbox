#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_subslice<T: PartialEq>(full_slice: &[T], sub_slice: &[T]) -> bool {
    // Iterate over each position in the full slice
    for i in 0..=(full_slice.len() - sub_slice.len()) {
        // Check if sub-slice matches starting from position i
        if full_slice[i..].starts_with(sub_slice) {
            return true;
        }
    }
    false
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else if _first_list.is_empty() && !_second_list.is_empty() {
        Comparison::Sublist
    } else if _first_list.len() > _second_list.len() && is_subslice(_first_list, _second_list) {
        Comparison::Superlist
    } else if _second_list.len() > _first_list.len() && is_subslice(_second_list, _first_list) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}
