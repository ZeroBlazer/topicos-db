pub fn median(vec: &Vec<f32>) -> f32 {
    let mut vec_cpy = vec.clone();
    vec_cpy.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let length = vec_cpy.len();
    let mid = length / 2;
    let mut ret = vec_cpy[mid];
    if length % 2 == 0 {
        ret += vec_cpy[mid - 1];
        ret /= 2.0;
    }

    ret
}

pub fn abs_standard_deviation(vec: &Vec<f32>) -> (f32, f32) {
    let median = median(vec);
    let mut asd = 0.0;

    for x in vec.iter() {
        asd += (x - median).abs();
    }

    (asd / vec.len() as f32, median)
}