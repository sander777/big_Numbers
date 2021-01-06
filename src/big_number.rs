use std::{cmp, collections::LinkedList, fmt::Display, ops};

use cmp::Ordering;

#[derive(Clone)]
pub struct BigNumber {
    sign: i8,
    number: LinkedList<i8>,
}

impl<'a> BigNumber {
    pub fn max(&'a self, other: &'a BigNumber) -> &'a BigNumber {
        if self > other {
            self
        } else {
            other
        }
    }
    pub fn min(&'a self, other: &'a BigNumber) -> &'a BigNumber {
        if self < other {
            self
        } else {
            other
        }
    }
    pub fn negative(&self) -> BigNumber {
        BigNumber {
            sign: self.sign * -1,
            number: self.number.clone(),
        }
    }
    pub fn pow(&self, p: &BigNumber) -> BigNumber {
        let mut p = p.clone();
        let mut base = self.clone();
        let mut res = BigNumber::from(1);
        let zero = BigNumber::from(0);
        let two = BigNumber::from(2);
        loop {
            if p.number.back().unwrap() % 2 != 0 {
                res *= &base;
            }
            p /= &two;
            if p == zero {
                break;
            }
            let temp = &base * &base;
            base = temp;
        }
        res
    }
    pub fn powi(&self, mut p: i32) -> BigNumber {
        let mut base = self.clone();
        let mut res = BigNumber::from(1);
        loop {
            if p & 1 == 1 {
                res *= &base;
            }
            p >>= 1;
            if p == 0 {
                break;
            }
            let temp = &base * &base;
            base = temp;
        }
        res
    }
    pub fn abs(&self) -> BigNumber {
        BigNumber {
            sign: 1,
            number: self.number.clone(),
        }
    }
    fn smaller_bigger_abs(l: &'a BigNumber, r: &'a BigNumber) -> (&'a BigNumber, &'a BigNumber) {
        if l.number.len() > r.number.len() {
            return (r, l);
        } else if r.number.len() > l.number.len() {
            return (l, &r);
        }
        let mut l_iter = l.number.iter();
        let mut r_iter = r.number.iter();
        for _ in 0..l.number.len() {
            let lin = match l_iter.next() {
                Some(n) => *n,
                None => 0,
            };
            let rin = match r_iter.next() {
                Some(n) => *n,
                None => 0,
            };
            if lin > rin {
                return (r, l);
            } else if rin > lin {
                return (l, r);
            }
        }
        (l, r)
    }
    fn normalize(mut self) -> BigNumber {
        while self.number.len() > 0 && *self.number.front().unwrap() == 0 {
            self.number.pop_front();
        }
        if self.number.len() == 0 {
            self.sign = 1;
        }

        self
    }
    fn cmp_abs(&self, other: &BigNumber) -> Option<Ordering> {
        if self.number.len() != other.number.len() {
            return Some(self.number.len().cmp(&other.number.len()));
        }
        let mut l_iter = self.number.iter();
        let mut r_iter = other.number.iter();
        for _ in 0..self.number.len() {
            let lin = match l_iter.next() {
                Some(n) => *n,
                None => 0,
            };
            let rin = match r_iter.next() {
                Some(n) => *n,
                None => 0,
            };
            if lin != rin {
                return Some(lin.cmp(&rin));
            }
        }
        Some(cmp::Ordering::Equal)
    }
}

impl From<i32> for BigNumber {
    fn from(n: i32) -> BigNumber {
        let mut num_list = LinkedList::new();
        for i in 0..how_many_digits(n) {
            num_list.push_front(get_i_digit(n, i));
        }
        BigNumber {
            sign: sign_of(n),
            number: num_list,
        }
    }
}

impl From<&str> for BigNumber {
    fn from(s: &str) -> BigNumber {
        let chars = s.as_bytes();
        let mut sign = 1;
        let mut num_list = LinkedList::new();
        let mut i = 0;
        if chars[0] as char == '-' {
            sign = -1;
            i = 1;
        }
        for j in i..chars.len() {
            num_list.push_back(chars[j] as i8 - 48);
        }
        BigNumber {
            sign,
            number: num_list,
        }
    }
}

macro_rules! gen_ops_impls {
    ($Op:ident, $op:ident, $o:tt) => {
        impl ops::$Op<BigNumber> for &BigNumber {
            type Output = BigNumber;
            fn $op(self, rhs: BigNumber) -> Self::Output {
                self $o &rhs
            }
        }

        impl ops::$Op<BigNumber> for BigNumber {
            type Output = BigNumber;
            fn $op(self, rhs: BigNumber) -> Self::Output {
                self $o &rhs
            }
        }

        impl ops::$Op<&BigNumber> for BigNumber {
            type Output = BigNumber;
            fn $op(self, rhs: &BigNumber) -> Self::Output {
                &self $o rhs
            }
        }
    };
}

macro_rules! gen_ops_assign_impls {
    ($Op:ident, $op:ident, $o:tt) => {
        impl ops::$Op<&BigNumber> for BigNumber {
            fn $op(&mut self, rhs: &BigNumber) {
                *self = &*self $o rhs;
            }
        }

        impl ops::$Op<BigNumber> for BigNumber {
            fn $op(&mut self, rhs: BigNumber) {
                *self = &*self $o rhs;
            }
        }
    };
}

impl ops::Add<&BigNumber> for &BigNumber {
    type Output = BigNumber;
    fn add(self, rhs: &BigNumber) -> Self::Output {
        let mut num_list = LinkedList::new();
        let max_n = self.number.len().max(rhs.number.len());
        let mut p = 0;
        let mut sign = 1;
        if self.sign == rhs.sign {
            let mut l_iter = self.number.iter().rev();
            let mut r_iter = rhs.number.iter().rev();
            for _ in 0..max_n {
                let l_in = match l_iter.next() {
                    Some(n) => *n,
                    None => 0,
                };
                let r_in = match r_iter.next() {
                    Some(n) => *n,
                    None => 0,
                };
                let ln = l_in;
                let rn = r_in;
                let (n, p_) = adder(rn, ln, p);
                p = p_;
                num_list.push_front(n.abs());
            }
            if p != 0 {
                num_list.push_front(p.abs());
            }
            sign = self.sign;
        } else {
            let (small, big) = BigNumber::smaller_bigger_abs(self, rhs);
            sign = big.sign;
            let mut small_iter = small.number.iter().rev();
            let mut big_iter = big.number.iter().rev();
            for _ in 0..max_n {
                let s_in = match small_iter.next() {
                    Some(n) => *n,
                    None => 0,
                };
                let b_in = match big_iter.next() {
                    Some(n) => *n,
                    None => 0,
                };
                let sn = -1 * s_in;
                let bn = b_in;
                let (n, p_) = adder(bn, sn, p);
                p = p_;
                num_list.push_front(n.abs());
            }
            if p != 0 {
                num_list.push_front(p.abs());
                sign = sign_of(p as i32);
            }
        }
        BigNumber {
            sign,
            number: num_list,
        }
        .normalize()
    }
}

gen_ops_impls!(Add, add, +);
gen_ops_assign_impls!(AddAssign, add_assign, +);

impl ops::Mul<&BigNumber> for &BigNumber {
    type Output = BigNumber;
    fn mul(self, rhs: &BigNumber) -> Self::Output {
        let mut res = BigNumber::from(0);
        let sign = self.sign * rhs.sign;
        let mut z = 0;
        for i in self.number.iter().rev() {
            let mut p = 0;
            let mut add = BigNumber::from(0);
            for _ in 0..z {
                add.number.push_front(0);
            }
            z += 1;
            for j in rhs.number.iter().rev() {
                let (n, p_) = multiplier(*i, *j, p);
                p = p_;
                add.number.push_front(n);
            }
            if p != 0 {
                add.number.push_front(p);
            }
            res += &add;
        }
        res.sign = sign;
        res.normalize()
    }
}

gen_ops_impls!(Mul, mul, *);
gen_ops_assign_impls!(MulAssign, mul_assign, *);

impl ops::Sub<&BigNumber> for &BigNumber {
    type Output = BigNumber;
    fn sub(self, rhs: &BigNumber) -> Self::Output {
        self + &rhs.negative()
    }
}

gen_ops_impls!(Sub, sub, -);
gen_ops_assign_impls!(SubAssign, sub_assign, -);

impl ops::Div<&BigNumber> for &BigNumber {
    type Output = BigNumber;
    fn div(self, rhs: &BigNumber) -> Self::Output {
        if *rhs == BigNumber::from(0) {
            panic!("dividing by 0")
        }
        let sign = self.sign * rhs.sign;
        match self.cmp_abs(rhs).unwrap() {
            Ordering::Less => BigNumber::from(0).normalize(),
            Ordering::Equal => {
                let mut res = BigNumber::from(1);
                res.sign = sign;
                res
            }
            Ordering::Greater => {
                let rhs = rhs.abs();
                let mut self_num = self.number.clone();
                let mut res = BigNumber::from(0);
                let mut temp_ = BigNumber::from(0);
                while self_num.len() > 0 {
                    temp_.number.push_back(self_num.pop_front().unwrap());
                    temp_ = temp_.normalize();
                    let (r, rem) = divider(&temp_, &rhs);
                    res.number.push_back(r);
                    temp_ = rem;
                }
                res.sign = sign;
                res.normalize()
            }
        }
    }
}

gen_ops_impls!(Div, div, /);
gen_ops_assign_impls!(DivAssign, div_assign, /);

impl cmp::PartialEq for BigNumber {
    fn eq(&self, other: &BigNumber) -> bool {
        if self.sign != other.sign {
            return false;
        }
        if self.number.len() != other.number.len() {
            return false;
        }
        let mut l_iter = self.number.iter().rev();
        let mut r_iter = other.number.iter().rev();
        for _ in 0..self.number.len() {
            let lin = match l_iter.next() {
                Some(n) => *n,
                None => 0,
            };
            let rin = match r_iter.next() {
                Some(n) => *n,
                None => 0,
            };
            if lin != rin {
                return false;
            }
        }

        true
    }
}

impl cmp::PartialOrd for BigNumber {
    fn partial_cmp(&self, other: &BigNumber) -> Option<cmp::Ordering> {
        if self.sign != other.sign {
            return Some(self.sign.cmp(&other.sign));
        }
        self.cmp_abs(other)
    }
}

impl Display for BigNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut write_string = String::new();
        if self.number.len() == 0 {
            write_string.push('0');
        } else {
            if self.sign == -1 {
                write_string.push('-');
            }
            for c in &self.number {
                write_string.push((*c as u8 + 48) as char);
            }
        }
        write!(f, "{}", write_string)
    }
}

fn get_i_digit(n: i32, i: usize) -> i8 {
    let n = n.abs();
    (n % 10i32.pow(i as u32 + 1) / 10i32.pow(i as u32)) as i8
}

fn sign_of(n: i32) -> i8 {
    if n >= 0 {
        1
    } else {
        -1
    }
}

fn how_many_digits(n: i32) -> usize {
    let mut s = 0usize;
    let n = n.abs();
    while n / 10i32.pow(s as u32) > 0 {
        s += 1;
    }
    s
}

fn adder(r: i8, l: i8, p: i8) -> (i8, i8) {
    let res = r + l + p;
    if res < 0 {
        (10 + res, -1)
    } else {
        (res % 10, res / 10)
    }
}

fn multiplier(r: i8, l: i8, p: i8) -> (i8, i8) {
    let res = r * l + p;
    (res % 10, res / 10)
}

fn divider(l: &BigNumber, r: &BigNumber) -> (i8, BigNumber) {
    let mut l = l.clone();
    let mut i = 0;
    while l.cmp_abs(r).unwrap() != Ordering::Less {
        l -= r;
        i += 1;
    }
    (i, l)
}
