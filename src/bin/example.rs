use pymath::{gamma, lgamma};

fn main() {
    let gamma_result = gamma(4.5).unwrap();
    let lgamma_result = lgamma(4.5).unwrap();
    println!("gamma(4.5) = {}", gamma_result);
    println!("lgamma(4.5) = {}", lgamma_result);
}
