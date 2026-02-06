pub fn bubble_sort(arr: &mut [i32]) -> &mut [i32] {
    let mut swapped = false;
    for i in 0..arr.len() {
        for j in 0..(arr.len() - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            println!("Noting Swapped");
            break;
        }
    }
    arr
}
