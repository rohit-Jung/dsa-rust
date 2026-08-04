use std::cmp::max;

pub fn solve(values: &[i32], weights: &[i32], sack_capacity: i32, idx: usize) -> i32 {
    if idx == 0 || sack_capacity <= 0 {
        return 0;
    }

    let mut take = 0;
    if weights[idx - 1] <= sack_capacity {
        take = values[idx - 1] + solve(values, weights, sack_capacity - weights[idx - 1], idx - 1)
    }

    let skip = solve(values, weights, sack_capacity, idx - 1);
    max(take, skip)
}

pub fn zeroone_knapsack_recursive(values: Vec<i32>, weights: Vec<i32>, sack_capacity: i32) -> i32 {
    solve(&values, &weights, sack_capacity, values.len())
}
