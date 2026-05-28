use rand::{rng, RngExt};
use rand::seq::SliceRandom;

pub fn random_vector(size: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = (0..(size - 1) / 2).collect();
    while vec.len() < size as usize {
        vec.push(10);
    }
    vec.shuffle(&mut rng());
    return vec;
}


pub fn random_index(n: usize) -> usize {
    let mut rng = rng();
    rng.random_range(0..n)
}