pub fn selection_sort(arr: &mut [i32]) -> &mut [i32] {
    for i in 0..arr.len() {
        let last_index = arr.len() - 1 - i;
        let max_index = find_max(arr, 0, last_index);

        arr.swap(max_index, last_index);
    }
    arr
}

pub fn find_max(arr: &[i32], start: usize, end: usize) -> usize {
    let mut max_index = start;
    for i in start..=end {
        if arr[i] > arr[max_index] {
            max_index = i
        }
    }
    max_index
}
