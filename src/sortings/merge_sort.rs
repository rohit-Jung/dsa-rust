// divide the array into half and half
// and then sort them and merge
pub fn merge_sort(arr: &mut [i32]) {
    divide(arr, 0, arr.len());
}

fn divide(arr: &mut [i32], s: usize, e: usize) {
    if e - s <= 1 {
        return;
    }

    let mid = s + (e - s) / 2;

    divide(arr, s, mid);
    divide(arr, mid, e);
    merge(arr, s, mid, e);
}

fn merge(arr: &mut [i32], s: usize, m: usize, e: usize) {
    let mut i = s;
    let mut j = m;
    let mut mix: Vec<i32> = Vec::with_capacity(e - s);

    while i < m && j < e {
        if arr[i] < arr[j] {
            mix.push(arr[i]);
            i += 1;
        } else {
            mix.push(arr[j]);
            j += 1;
        }
    }

    while i < m {
        mix.push(arr[i]);
        i += 1;
    }

    while j < e {
        mix.push(arr[j]);
        j += 1;
    }

    arr[s..e].copy_from_slice(&mix);
    // for l in 0..mix.len() {
    //     arr[s + l] = mix[l]
    // }
}
