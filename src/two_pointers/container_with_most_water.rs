use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    if height.len() <= 1 {
        return 0;
    }

    let mut l = 0;
    let mut r = height.len() - 1;

    let mut max_a = 0;
    // we do not want the water to overflow so we keep bigger side on
    // one side  at any given time
    while l < r {
        let area = (r - l) as i32 * min(height[l], height[r]);
        max_a = max(max_a, area);

        if height[l] <= height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    max_a
}
