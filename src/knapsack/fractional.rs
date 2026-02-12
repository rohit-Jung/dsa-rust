pub fn fractional_knapsack(values: Vec<i32>, weights: Vec<i32>, sack_capacity: i32) {
    let mut profit = 0.00;
    let mut cap = sack_capacity;
    let mut per_values: Vec<(f64, i32, i32)> = Vec::new();

    for i in 0..values.len() {
        let per_value = values[i] as f64 / weights[i] as f64;
        per_values.push((per_value, values[i], weights[i]));
    }

    per_values.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    for &(ratio, val, weight) in &per_values {
        if cap == 0 {
            break;
        }
        if cap >= weight {
            profit += val as f64;
            cap -= weight;
        } else {
            profit += ratio * cap as f64;
            break;
        }
    }

    println!("Max profit is {profit}");
}
