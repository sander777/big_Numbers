mod big_number;

use std::time::{Duration, Instant};

use big_number::BigNumber;

fn main() {
    println!(
        "{:?}",
        mesure(|| {
            let a = BigNumber::from(
                "23984871283974891273453245234523453123423409812083759021834072138421093848217395742037404",
            );
            let mut res = BigNumber::from(1);
            for i in 0..100 {
                println!("{}", i);
                res *= &a;
            }
            println!("{}^50 = {}", a, res);
        })
    );
}

fn mesure<F>(work: F) -> Duration
where
    F: Fn() -> (),
{
    let start = Instant::now();
    work();
    Instant::now() - start
}
