pub fn quicksort<T: PartialOrd>(vec: &mut [T]) {
    if vec.len() < 2 {
        return;
    }

    _quicksort(vec, 0, vec.len() - 1);
}

fn _quicksort<T: PartialOrd>(vec: &mut [T], lower: usize, upper: usize) {
    let pivot_index = first_pivot_index(vec, lower, upper);
    let partition_index = partition(vec, pivot_index, lower, upper);
    if partition_index > lower {
        _quicksort(vec, lower, partition_index - 1);
    }
    if partition_index < upper {
        _quicksort(vec, partition_index + 1, upper);
    }
}

fn first_pivot_index<T>(_vec: &mut [T], lower: usize, _upper: usize) -> usize {
    return lower;
}

fn middle_pivot_index<T>(_vec: &mut [T], lower: usize, upper: usize) -> usize {
    let middle_float = (upper + lower) as f64 / 2.0;
    return middle_float.round() as usize;
}

fn last_pivot_index<T>(_vec: &mut [T], _lower: usize, upper: usize) -> usize {
    return upper;
}

fn partition<T: PartialOrd>(
    vec: &mut [T],
    pivot_index: usize,
    lower: usize,
    upper: usize,
) -> usize {
    if lower >= upper {
        // FIXME what do I return here?
        return lower;
    }

    vec.swap(pivot_index, upper);

    let mut division_index = lower;
    for i in lower..upper {
        if vec[i] < vec[upper] {
            vec.swap(i, division_index);
            division_index += 1;
        }
    }

    vec.swap(upper, division_index);

    division_index
}

#[cfg(test)]
mod tests {
    use crate::quicksort::{partition, quicksort};

    #[test]
    fn test_partition_1() {
        let mut vec = vec![5, 7, 8, 2];
        let vec_len = vec.len() - 1;
        let partition_index = partition(&mut vec, 0, 0, vec_len);
        assert_eq!(vec, [2, 5, 8, 7]);
        assert_eq!(partition_index, 1);
    }

    #[test]
    fn test_partition_2() {
        let mut vec = vec![5, 7, 8, 2];
        let vec_len = vec.len() - 1;
        let partition_index = partition(&mut vec, 1, 0, vec_len);
        assert_eq!(vec, [5, 2, 7, 8]);
        assert_eq!(partition_index, 2);
    }

    #[test]
    fn test_partition_3() {
        let mut vec = vec![5, 7, 8, 2];
        let vec_len = vec.len() - 1;
        let partition_index = partition(&mut vec, 2, 0, vec_len);
        assert_eq!(vec, [5, 7, 2, 8]);
        assert_eq!(partition_index, 3);
    }

    #[test]
    fn test_partition_4() {
        let mut vec = vec![5, 7, 8, 2];
        let vec_len = vec.len() - 1;
        let partition_index = partition(&mut vec, 3, 0, vec_len);
        assert_eq!(vec, [2, 7, 8, 5]);
        assert_eq!(partition_index, 0);
    }

    #[test]
    fn test_partition_5() {
        let mut vec = vec![9, 5, 7, 8, 2];
        let vec_len = vec.len() - 1;
        let partition_index = partition(&mut vec, 3, 0, vec_len);
        assert_eq!(vec, [5, 7, 2, 8, 9]);
        assert_eq!(partition_index, 3);
    }

    #[test]
    fn test_partition_6() {
        let mut vec = vec![5, 4, 3, 2, 1];
        let vec_len = vec.len() - 1;
        let partition_index = partition(&mut vec, 0, 0, vec_len);
        assert_eq!(vec, [1, 4, 3, 2, 5]);
        assert_eq!(partition_index, 4);
    }

    #[test]
    fn test_quicksort() {
        let mut vec = vec![5, 4, 3, 2, 1];
        quicksort(&mut vec);
        assert_eq!(vec, [1, 2, 3, 4, 5]);
    }

    // FIXME add more tests for edge cases
    // FIXME add benchmark to compare this against the stdlib
}
