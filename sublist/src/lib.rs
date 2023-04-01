#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match true {
        _ if { first_list.len() > second_list.len() } => {
            if contains(first_list, second_list) {
                return Comparison::Superlist;
            }
            Comparison::Unequal
        }
        _ if { second_list.len() > first_list.len() } => {
            if contains(second_list, first_list) {
                return Comparison::Sublist;
            }
            Comparison::Unequal
        }
        _ => {
            if contains(first_list, second_list) {
                return Comparison::Equal;
            }
            Comparison::Unequal
        }
    }
}

fn contains<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.len() == 0 || second_list.len() == 0 {
        return true;
    }

    'main: for (index, _) in first_list.iter().enumerate() {
        if first_list.len() - index < second_list.len() {
            return false;
        }

        for (f, s) in first_list[index..index + second_list.len()]
            .iter()
            .zip(second_list)
        {
            if f != s {
                continue 'main;
            }
        }
        return true;
    }

    false
}
