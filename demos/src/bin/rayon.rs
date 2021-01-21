use rayon::prelude::*;

fn sum_squares(n: usize) -> usize {
    (1..=n).map(|i| i * i).sum()
}

fn sum_squares_parallel(n: usize) -> usize {
    (1..=n).into_par_iter().map(|i| i * i).sum()
}

fn main() {
    println!("{}", sum_squares(1000000));
    println!("{}", sum_squares_parallel(1000000));
}
