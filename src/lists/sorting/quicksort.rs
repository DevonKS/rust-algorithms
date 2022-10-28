pub fn quick_sort<T: PartialOrd>(vec: &mut [T]) {
    if vec.len() < 2 {
        return;
    }

    _quick_sort(vec);
}

fn _quick_sort<T: PartialOrd>(vec: &mut [T]) {
    let vec_len = vec.len();
    if vec_len < 2 {
        return;
    }

    let pivot_index = first_pivot_index(vec);
    let partition_index = partition(vec, pivot_index);
    _quick_sort(&mut vec[0..partition_index]);
    _quick_sort(&mut vec[partition_index + 1..vec_len]);
}

fn first_pivot_index<T>(_vec: &mut [T]) -> usize {
    return 0;
}

#[allow(dead_code)]
fn middle_pivot_index<T>(vec: &mut [T]) -> usize {
    let middle_float = (vec.len()) as f64 / 2.0;
    return middle_float.round() as usize;
}

#[allow(dead_code)]
fn last_pivot_index<T>(vec: &mut [T]) -> usize {
    return vec.len() - 1;
}

fn partition<T: PartialOrd>(vec: &mut [T], pivot_index: usize) -> usize {
    let last_index = vec.len() - 1;
    vec.swap(pivot_index, last_index);

    let mut division_index = 0;
    for i in 0..last_index {
        if vec[i] < vec[last_index] {
            vec.swap(i, division_index);
            division_index += 1;
        }
    }

    vec.swap(last_index, division_index);

    division_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_1() {
        let mut vec = vec![5, 7, 8, 2];
        let partition_index = partition(&mut vec, 0);
        assert_eq!(vec, [2, 5, 8, 7]);
        assert_eq!(partition_index, 1);
    }

    #[test]
    fn test_partition_2() {
        let mut vec = vec![5, 7, 8, 2];
        let partition_index = partition(&mut vec, 1);
        assert_eq!(vec, [5, 2, 7, 8]);
        assert_eq!(partition_index, 2);
    }

    #[test]
    fn test_partition_3() {
        let mut vec = vec![5, 7, 8, 2];
        let partition_index = partition(&mut vec, 2);
        assert_eq!(vec, [5, 7, 2, 8]);
        assert_eq!(partition_index, 3);
    }

    #[test]
    fn test_partition_4() {
        let mut vec = vec![5, 7, 8, 2];
        let partition_index = partition(&mut vec, 3);
        assert_eq!(vec, [2, 7, 8, 5]);
        assert_eq!(partition_index, 0);
    }

    #[test]
    fn test_partition_5() {
        let mut vec = vec![9, 5, 7, 8, 2];
        let partition_index = partition(&mut vec, 3);
        assert_eq!(vec, [5, 7, 2, 8, 9]);
        assert_eq!(partition_index, 3);
    }

    #[test]
    fn test_partition_6() {
        let mut vec = vec![5, 4, 3, 2, 1];
        let partition_index = partition(&mut vec, 0);
        assert_eq!(vec, [1, 4, 3, 2, 5]);
        assert_eq!(partition_index, 4);
    }

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        quick_sort(&mut res);
        assert_eq!(res, vec!["a", "bb", "cc", "d"]);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        quick_sort(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        quick_sort(&mut res);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn two_elements_sorted() {
        let mut res = vec![1, 2];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2]);
    }

    #[test]
    fn two_elements_unsorted() {
        let mut res = vec![2, 1];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted_2() {
        let mut vec = vec![5, 4, 3, 2, 1];
        quick_sort(&mut vec);
        assert_eq!(vec, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn repeated_elements() {
        let mut arr = vec![542, 542, 542, 542];
        quick_sort(&mut arr);
        assert_eq!(&arr, &vec![542, 542, 542, 542]);
    }

    // FIXME add benchmark to compare this against the stdlib
}
