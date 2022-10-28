pub fn heap_sort<T: PartialOrd>(vec: &mut [T]) {
    max_heapify(vec);
    for i in (1..vec.len()).rev() {
        vec.swap(0, i);
        max_heap_sift_down(&mut vec[0..i], 0);
    }
}

pub fn max_heapify<T: PartialOrd>(vec: &mut [T]) {
    for i in 1..vec.len() {
        max_heap_sift_up(&mut vec[0..i + 1], i)
    }
}

// FIXME does it makes sense to take i as an arg? or are we always sifting the last element?
pub fn max_heap_sift_up<T: PartialOrd>(vec: &mut [T], i: usize) {
    if i == 0 {
        return;
    }

    let mut current_index = i;
    let mut parent_index = (current_index - 1) / 2;
    loop {
        if vec[current_index] > vec[parent_index] {
            vec.swap(current_index, parent_index);
            if parent_index == 0 {
                break;
            }
            current_index = parent_index;
            parent_index = (current_index - 1) / 2;
        } else {
            break;
        }
    }
}

// FIXME does it makes sense to take i as an arg? or are we always sifting the first element?
pub fn max_heap_sift_down<T: PartialOrd>(vec: &mut [T], i: usize) {
    let max_index = vec.len() - 1;

    let mut current_index = i;
    let mut left_child_index = (current_index * 2) + 1;
    let mut right_child_index = left_child_index + 1;
    loop {
        let have_left_child = left_child_index <= max_index;
        let have_right_child = right_child_index <= max_index;

        let mut swapped = false;

        // FIXME gotta be a better way to organise this logic
        if have_left_child && have_right_child {
            if vec[left_child_index] > vec[current_index]
                && vec[left_child_index] > vec[right_child_index]
            {
                vec.swap(current_index, left_child_index);
                current_index = left_child_index;
                swapped = true;
            } else if vec[right_child_index] > vec[current_index]
                && vec[right_child_index] > vec[left_child_index]
            {
                vec.swap(current_index, right_child_index);
                current_index = right_child_index;
                swapped = true;
            }
        } else if have_left_child && vec[left_child_index] > vec[current_index] {
            vec.swap(current_index, left_child_index);
            current_index = left_child_index;
            swapped = true;
        } else if have_right_child && vec[right_child_index] > vec[current_index] {
            vec.swap(current_index, right_child_index);
            current_index = right_child_index;
            swapped = true;
        }

        left_child_index = (current_index * 2) + 1;
        right_child_index = left_child_index + 1;

        if !swapped || (left_child_index > max_index && right_child_index > max_index) {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_max_heap() {
        let mut vec = vec![4, 5, 5, 6, 6, 7, 9];
        max_heapify(&mut vec);
        assert_eq!(vec, [9, 6, 7, 4, 5, 5, 6]);
    }

    #[test]
    fn test_create_max_heap_2() {
        let mut vec = vec![6, 5, 3, 1, 8, 7, 2, 4];
        max_heapify(&mut vec);
        assert_eq!(vec, [8, 6, 7, 4, 5, 3, 2, 1]);
    }

    #[test]
    fn test_heapsort() {
        let mut vec = vec![6, 5, 3, 1, 8, 7, 2, 4];
        heap_sort(&mut vec);
        assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        heap_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        heap_sort(&mut res);
        assert_eq!(res, vec!["a", "bb", "cc", "d"]);
    }

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = Vec::new();
        heap_sort(&mut arr);
        assert_eq!(&arr, &[]);
    }

    #[test]
    fn single_element() {
        let mut arr = vec![1];
        heap_sort(&mut arr);
        assert_eq!(&arr, &[1]);
    }

    #[test]
    fn two_elements_sorted() {
        let mut res = vec![1, 2];
        heap_sort(&mut res);
        assert_eq!(res, vec![1, 2]);
    }

    #[test]
    fn two_elements_unsorted() {
        let mut res = vec![2, 1];
        heap_sort(&mut res);
        assert_eq!(res, vec![1, 2]);
    }

    #[test]
    fn sorted_array() {
        let mut arr = vec![1, 2, 3, 4];
        heap_sort(&mut arr);
        assert_eq!(&arr, &[1, 2, 3, 4]);
    }

    #[test]
    fn unsorted_array() {
        let mut arr = vec![3, 4, 2, 1];
        heap_sort(&mut arr);
        assert_eq!(&arr, &[1, 2, 3, 4]);
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr = vec![3, 4, 2, 1, 7];
        heap_sort(&mut arr);
        assert_eq!(&arr, &[1, 2, 3, 4, 7]);
    }

    #[test]
    fn repeated_elements() {
        let mut arr = vec![542, 542, 542, 542];
        heap_sort(&mut arr);
        assert_eq!(&arr, &vec![542, 542, 542, 542]);
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        heap_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted_2() {
        let mut vec = vec![5, 4, 3, 2, 1];
        heap_sort(&mut vec);
        assert_eq!(vec, [1, 2, 3, 4, 5]);
    }
}
