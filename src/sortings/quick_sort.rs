// done by selection of pivot element and putting it into right place
pub fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_idx = lomuto_partition(arr, 0, arr.len() - 1);
    // let pivot_idx = hoare_partition(arr, 0, arr.len() - 1);
    quick_sort(&mut arr[..=pivot_idx]);
    quick_sort(&mut arr[pivot_idx + 1..]);
}

// hoare_partition: compare and swap
fn hoare_partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[low];

    let mut i = low as isize - 1;
    let mut j = high as isize + 1;

    loop {
        //mimicking do while loop
        loop {
            i += 1;
            if arr[i as usize] >= pivot {
                break;
            }
        }

        loop {
            j -= 1;
            if arr[j as usize] < pivot {
                break;
            }
        }

        // if exceeds then
        if i >= j {
            return j as usize;
        }

        arr.swap(i as usize, j as usize);
    }
}

// easier version of pivot finding correct position
fn lomuto_partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] > pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, high);
    i
}
