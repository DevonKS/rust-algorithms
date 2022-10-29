// FIXME Should I make it put the result in the original vec?
// FIXME Should I have the trait Copy or Clone here?
pub fn merge_sort<T: PartialOrd + Copy>(vec: &[T]) -> Vec<T> {
    let vec_len = vec.len();
    if vec_len <= 1 {
        return vec.to_vec();
    }

    let half = vec_len / 2;
    let left = merge_sort(&vec[..half].to_vec());
    let right = merge_sort(&vec[half..].to_vec());
    merge(&left, &right)
}

// FIXME Should I have the trait Copy or Clone here?
fn merge<T: PartialOrd + Copy>(a: &[T], b: &[T]) -> Vec<T> {
    let merged_len = a.len() + b.len();
    let mut v = Vec::with_capacity(merged_len);

    let mut a_index = 0;
    let mut b_index = 0;
    while a_index < a.len() || b_index < b.len() {
        // FIXME is there a better way to handle these cases?
        if a_index >= a.len() {
            v.push(b[b_index]);
            b_index += 1;
        } else if b_index >= b.len() {
            v.push(a[a_index]);
            a_index += 1;
        } else if a[a_index] <= b[b_index] {
            v.push(a[a_index]);
            a_index += 1;
        } else {
            v.push(b[b_index]);
            b_index += 1;
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_merge_sort() {
        let res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let result = merge_sort(&res);
        assert_eq!(result, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn basic_string() {
        let res = vec!["a", "bb", "d", "cc"];
        let result = merge_sort(&res);
        assert_eq!(result, ["a", "bb", "cc", "d"]);
    }

    #[test]
    fn empty() {
        let res = Vec::<u8>::new();
        let result = merge_sort(&res);
        assert_eq!(result, []);
    }

    #[test]
    fn one_element() {
        let res = vec![1];
        let result = merge_sort(&res);
        assert_eq!(result, [1]);
    }

    #[test]
    fn two_elements_sorted() {
        let res = vec![1, 2];
        let result = merge_sort(&res);
        assert_eq!(result, [1, 2]);
    }

    #[test]
    fn two_elements_unsorted() {
        let res = vec![2, 1];
        let result = merge_sort(&res);
        assert_eq!(result, [1, 2]);
    }

    #[test]
    fn pre_sorted() {
        let res = vec![1, 2, 3, 4];
        let result = merge_sort(&res);
        assert_eq!(result, [1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted() {
        let res = vec![4, 3, 2, 1];
        let result = merge_sort(&res);
        assert_eq!(result, [1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted_2() {
        let vec = vec![5, 4, 3, 2, 1];
        let result = merge_sort(&vec);
        assert_eq!(result, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn repeated_elements() {
        let arr = vec![542, 542, 542, 542];
        let result = merge_sort(&arr);
        assert_eq!(result, [542, 542, 542, 542]);
    }

    // FIXME add benchmark to compare this against the stdlib
}
