fn binary_search_recursive<T: PartialOrd + PartialEq>(arr: &[T], search_value: T) -> Option<usize> {
    if arr.len() == 0 {
        return None;
    }

    binary_search_recursive_start(arr, search_value, 0, arr.len())
}

fn binary_search_recursive_start<T: PartialOrd + PartialEq>(
    arr: &[T],
    search_value: T,
    low: usize,  // inclusive
    high: usize, // exclusive
) -> Option<usize> {
    // base case
    if low == high {
        return None;
    }

    let middle_i = low + ((high - low) / 2);
    let middle = &arr[middle_i];

    if *middle == search_value {
        return Some(middle_i);
    } else if search_value < *middle {
        return binary_search_recursive_start(arr, search_value, low, middle_i);
    } else {
        return binary_search_recursive_start(arr, search_value, middle_i + 1, high);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_empty_array() {
        let arr: Vec<i32> = vec![];
        assert_eq!(binary_search_recursive(&arr, 5), None);
    }

    #[test]
    fn test_binary_search_recursive_single_element_array() {
        let arr = vec![10];
        assert_eq!(binary_search_recursive(&arr, 10), Some(0));
        assert_eq!(binary_search_recursive(&arr, 5), None);
    }

    #[test]
    fn test_binary_search_recursive_even_length_array() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(binary_search_recursive(&arr, 3), Some(2));
        assert_eq!(binary_search_recursive(&arr, 6), Some(5));
        assert_eq!(binary_search_recursive(&arr, 7), None);
    }

    #[test]
    fn test_binary_search_recursive_odd_length_array() {
        let arr = vec![10, 20, 30, 40, 50];
        assert_eq!(binary_search_recursive(&arr, 30), Some(2));
        assert_eq!(binary_search_recursive(&arr, 10), Some(0));
        assert_eq!(binary_search_recursive(&arr, 45), None);
    }

    #[test]
    fn test_binary_search_recursive_strings() {
        let arr = vec!["apple", "banana", "orange", "pear"];
        assert_eq!(binary_search_recursive(&arr, "orange"), Some(2));
        assert_eq!(binary_search_recursive(&arr, "grape"), None);
    }
}
