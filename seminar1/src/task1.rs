pub fn quick_sort(arr: Vec<impl PartialOrd>) -> Vec<i32> {
    
}

#[cfg(test)]
mod test {
    use super::quick_sort;
    use crate::test_helpers::generate_random_list;

    #[test]
    fn test_file() {
        let r = generate_random_list(10);
        assert_eq!(quick_sort(r.0), r.1);
    }
}
