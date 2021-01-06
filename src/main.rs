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
            println!("{}^100 = {}", a, a.pow(&BigNumber::from(100)));
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

extern crate rand;
use rand::prelude::*;
use std::{fs, io::prelude::*, thread};

fn test() {
    let mut rng = rand::thread_rng();
    let r = 10000;
    let mut file = fs::File::create("errors.txt").unwrap();
    for _ in 0..10000 {
        let a = rng.gen_range(-r..r);
        let b = rng.gen_range(-r..r);
        let a_b = BigNumber::from(a);
        let b_b = BigNumber::from(b);
        println!("{} == {}", a + b, &a_b + &b_b);
        if format!("{}", a + b) != format!("{}", &a_b + &b_b) {
            file.write(
                format!(
                    "({} + {}   ,  {} + {}) == ({}, {})\n",
                    a,
                    b,
                    a_b,
                    b_b,
                    a + b,
                    &a_b + &b_b
                )
                .as_bytes(),
            )
            .unwrap();
        }

        println!("{} == {}", a - b, &a_b - &b_b);
        if format!("{}", a - b) != format!("{}", &a_b - &b_b) {
            file.write(
                format!(
                    "({} - {}   ,  {} - {} ) == ({}, {})\n",
                    a,
                    b,
                    a_b,
                    b_b,
                    a - b,
                    &a_b - &b_b
                )
                .as_bytes(),
            )
            .unwrap();
        }
        println!("{} == {}", a * b, &a_b * &b_b);
        if format!("{}", a * b) != format!("{}", &a_b * &b_b) {
            file.write(
                format!(
                    "({} * {}   ,  {} * {} ) == ({}, {})\n",
                    a,
                    b,
                    a_b,
                    b_b,
                    a * b,
                    &a_b * &b_b
                )
                .as_bytes(),
            )
            .unwrap();
        }
        if b != 0 {
            println!("{} == {}", a / b, &a_b / &b_b);
            if format!("{}", a / b) != format!("{}", &a_b / &b_b) {
                file.write(
                    format!(
                        "({} / {}   ,  {} / {} ) == ({}, {})\n",
                        a,
                        b,
                        a_b,
                        b_b,
                        a / b,
                        &a_b / &b_b
                    )
                    .as_bytes(),
                )
                .unwrap();
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}
