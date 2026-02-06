pub fn fractional_knapsack(values: Vec<i32>, weights: Vec<i32>, sack_capacity: i32) {
    let mut items: Vec<(i32, i32, f64)> = Vec::new();

    let mut capacity = sack_capacity as f64;
    let mut profit = 0.00;

    for i in 0..values.len() {
        let ratio = weights[i] as f64 * values[i] as f64;
        items.push((values[i], weights[i], ratio));
    }

    // sort the per values  (bubble sort ?) rust uses stable sort ?
    items.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    // start picking max
    for &(val, weight, ratio) in &items {
        if capacity == 0.0 {
            break;
        }

        if capacity >= weight as f64 {
            capacity -= weight as f64;
            profit += val as f64;
        } else {
            profit += ratio * capacity;
            // set capaity to 0
            capacity = 0.00;
        }
    }

    println!("Maximum profit {:.2}", profit);
}
