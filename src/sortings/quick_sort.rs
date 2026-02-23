// done by selection of pivot element and putting it into right place

pub fn quick_sort(arr: &mut [i32]) {
    if arr.len() > 1 {
        sort(arr, 0, arr.len() - 1);
    }
}

fn sort(arr: &mut [i32], s: usize, e: usize) {
    if s >= e {
        return;
    }

    let mut i = s;
    let mut j = e;
    let mid = s + (e - s) / 2;
    let pivot = arr[mid];

    while i <= j {
        while arr[i] < pivot {
            i += 1;
        }

        while arr[j] > pivot {
            j -= 1;
        }

        if i <= j {
            arr.swap(i, j);
            i += 1;
            if j == 0 {
                break;
            };
            j -= 1;
        }
    }

    sort(arr, s, j);
    sort(arr, i, e);
}
