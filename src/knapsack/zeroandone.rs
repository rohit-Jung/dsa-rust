pub fn print_arr(arr: Vec<Vec<i32>>) {
    for row in arr.iter() {
        println!("{:?}", row);
    }
}

// i - 1 when you as you are using the first row as 0
// what value to take when exclude and what when include ?
// j -> capacity ? i -> items available ?   j >= weights[of the item]
// dp[i][j] = max value using the first i items with capacity j
pub fn zeroone_knapsack(values: Vec<i32>, weights: Vec<i32>, sack_capacity: i32) -> i32 {
    let rows: usize = weights.len() + 1;
    let cols: usize = (sack_capacity + 1) as usize;

    let mut dp = vec![vec![0; cols]; rows];

    for i in 1..rows {
        for w in 0..cols {
            let exclude = dp[i - 1][w];
            let include = if w >= weights[i - 1] as usize {
                dp[i - 1][w - weights[i - 1] as usize] + values[i - 1]
            } else {
                0
            };

            dp[i][w] = exclude.max(include);
        }
    }

    let max_profit = dp[rows - 1][cols - 1];
    println!("MAX {}", dp[rows - 1][cols - 1]);

    max_profit
}
