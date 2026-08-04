pub fn trap(height: Vec<i32>) -> i32 {
    let mut trapped_water = 0;

    let mut l = 0;
    let mut r = height.len() - 1;

    let mut max_left = 0;
    let mut max_right = 0;

    // we go through the min height to hold water ?
    while l < r {
        if height[l] < height[r] {
            if height[l] >= max_left {
                max_left = height[l];
            } else {
                trapped_water += max_left - height[l];
            }

            l += 1;
        } else {
            if height[r] >= max_right {
                max_right = height[r];
            } else {
                trapped_water += max_right - height[r];
            }

            r -= 1;
        }
    }

    trapped_water
}
