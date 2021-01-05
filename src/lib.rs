pub mod big_number;
pub use big_number::*;

#[cfg(test)]
mod test {
    extern crate rand;
    use crate::big_number::BigNumber;
    use rand::prelude::*;
    use std::{fs, io::prelude::*, thread, time::Duration};
    #[test]
    fn test() {
        let mut rng = rand::thread_rng();
        let r = 10000;
        let mut file = fs::File::create("errors.txt").unwrap();
        for _ in 0..10000 {
            let a = rng.gen_range(-r..r);
            let b = rng.gen_range(-r..r);
            let a_b = BigNumber::from(a);
            let b_b = BigNumber::from(b);
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
            thread::sleep(Duration::from_millis(000));
        }
    }
}
