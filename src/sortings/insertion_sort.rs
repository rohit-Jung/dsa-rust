//IMP: we go from 1 in j not 0 as 0 can lead to [0-1]
pub fn insertion_sort(arr: &mut [i32]) -> &mut [i32] {
    // sort 2, 3, 4 and all elements like that
    for i in 0..arr.len() {
        // we go from 1 here not 0 as 0 can lead to [0-1]
        for j in (1..=i).rev() {
            if arr[j - 1] > arr[j] {
                arr.swap(j, j - 1);
            } else {
                break;
            }
        }
    }

    arr
}
