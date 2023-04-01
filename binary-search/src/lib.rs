pub fn find<T: Ord>(array: &[T], key: T) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }

    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        if array[mid] == key {
            return Some(mid);
        } else if array[mid] < key {
            low = mid + 1;
        } else if mid > 0 {
            high = mid - 1;
        } else {
            break;
        }
    }
    None
}
