pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sorted_nums: Vec<i32> = nums.clone();
    sorted_nums.sort(); // logn
    let mut solution: Vec<Vec<i32>> = Vec::new();

    for (idx, &num) in sorted_nums.iter().enumerate() {
        if idx > 0 && sorted_nums[idx] == sorted_nums[idx - 1] {
            continue;
        };

        let mut l = idx + 1;
        let mut r = sorted_nums.len() - 1;
        while l < r {
            let three_sum = sorted_nums[l] + sorted_nums[r] + num;
            if three_sum == 0 {
                solution.push(vec![sorted_nums[l], sorted_nums[r], num]);

                l += 1;
                r -= 1;

                // skip the duplicates
                while l < r && sorted_nums[l] == sorted_nums[l - 1] {
                    l += 1;
                }

                while l < r && sorted_nums[r] == sorted_nums[r + 1] {
                    r -= 1;
                }
            } else if three_sum > 0 {
                r -= 1;
            } else {
                l += 1;
            }
        }
    }

    solution
}
