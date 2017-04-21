pub fn manhattan_dist(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    if x.len() != y.len() {
        panic!("Should compare vectors of same size");
    }
    let mut distance = 0.0;

    for i in 0..x.len() {
        if x[i] > 0.0 && y[i] > 0.0 {
            distance += (x[i] - y[i]).abs();
        }
    }

    distance
}

pub fn euclidian_dist(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    if x.len() != y.len() {
        panic!("Should compare vectors of same size");
    }
    let mut distance = 0.0;

    for i in 0..x.len() {
        distance += (x[i] - y[i]).powf(2.0);
    }

    distance.sqrt()
}

pub fn cosine_dist(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    if x.len() != y.len() {
        panic!("Should compare vectors of same size");
    }

    let mut dot = 0.0;
    let mut den_a = 0.0;
    let mut den_b = 0.0;

    for i in 0..x.len() {
        dot += x[i] * y[i];
        den_a += x[i] * x[i];
        den_b += y[i] * y[i];
    }
    dot / (den_a.sqrt() * den_b.sqrt())
}

pub fn pearson_coef(x: &Vec<f32>, y: &Vec<f32>) -> f32 {
    if x.len() != y.len() {
        panic!("Should compare vectors of same size");
    }

    let n_dims = x.len();

    let mut prod_xy = 0.0;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;

    for i in 0..n_dims {
        sum_x += x[i];
        sum_y += y[i];
        prod_xy += x[i] * y[i];
    }

    let avg_sqr_x = sum_x.powf(2.0) / n_dims as f32;
    let avg_sqr_y = sum_y.powf(2.0) / n_dims as f32;
    let mut sqr_diff_x = 0.0;
    let mut sqr_diff_y = 0.0;

    for i in 0..x.len() {
        sqr_diff_x += x[i].powf(2.0);
        sqr_diff_y += y[i].powf(2.0);
    }

    (prod_xy - (sum_x * sum_y) / n_dims as f32) /
    ((sqr_diff_x - avg_sqr_x).sqrt() * (sqr_diff_y - avg_sqr_y).sqrt())
}