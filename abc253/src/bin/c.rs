#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::{max, min};
use std::collections::{BinaryHeap, BTreeMap, BTreeSet, VecDeque};
use std::iter::once;
use ac_library::{Additive, Dsu, DynamicModInt, Max, Min, ModInt1000000007, ModInt998244353, Monoid, Multiplicative, Segtree};
use bstr::ByteSlice;
use easy_ext::ext;
use indexing::container_traits::Pushable;
use itertools::Itertools;
use itertools_num::ItertoolsNum;
use num_integer::{gcd, gcd_lcm};
use num_rational::Ratio;
use num_traits::{FromPrimitive, ToPrimitive};
use omniswap::swap;
use proconio::{fastout, input};
use proconio::marker::{Bytes, Usize1};
use rustc_hash::{FxHashMap, FxHashSet};
use superslice::Ext;

fn main() {
    input! {
        q: usize
    }
    let mut query = vec![];
    for _ in 0..q {
        input! { t: usize }
        match t {
            1 => {
                input! { x: usize }
                query.push((t, x, 0));
            }
            2 => {
                input! { x: usize, c: usize }
                query.push((t, x, c));
            }
            _ => {
                query.push((t, 0, 0));
            }
        }
    }

    let mut map = BTreeMap::new();
    for (t, x, c) in query {
        match t {
            1 => {
                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                if !map.contains_key(&x) { continue }
                if map[&x] <= c {
                    map.remove(&x);
                } else {
                    *map.get_mut(&x).unwrap() -= c;
                }
            }
            _ => {
                println!("{}", map.keys().next_back().unwrap() - map.keys().next().unwrap());
            }
        }
    }
}

const INF: usize = 1_000_000_000_000_000_000;

const DIR4: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const DIR8: [(isize, isize); 8] = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];

#[ext]
impl usize {
    fn is_prime(&self) -> bool {
        if *self == 0 || *self == 1 { return false }
        (2..).take_while(|&x| x * x <= *self).all(|x| *self % x == 0)
    }

    fn divisors(&self) -> Vec<usize> {
        let mut div = vec![];
        for i in (1..).take_while(|&x| x * x <= *self) {
            if *self % i != 0 { continue }
            div.push(i);
            if *self / i != i { div.push(*self / i) }
        }
        div
    }

    fn factors(&self) -> Vec<(usize, usize)> {
        let mut fact = vec![];
        let mut n = *self;
        for i in 2..=*self {
            if i * i > n { break }
            if n % i != 0 { continue }
            let mut e = 0;
            while n % i == 0 {
                e += 1;
                n /= i;
            }
            fact.push((i, e));
        }
        if n != 1 { fact.push((n, 1)) }
        fact
    }
}

#[ext]
impl bool {
    fn yes_no(&self) -> &str {
        if *self { "Yes" } else { "No" }
    }
}

fn eratosthenes(n: usize) -> Vec<bool> {
    let mut vec = vec![true; n + 1];
    vec[0] = false;
    vec[1] = false;
    for i in (2..).take_while(|&x| x * x <= n) {
        if !vec[i] { continue }
        for j in (2..).take_while(|&j| i * j <= n) {
            vec[i * j] = false;
        }
    }
    vec
}

fn compress<T: Ord>(vec: &[T]) -> Vec<usize> {
    let set = vec.iter().collect::<BTreeSet<_>>();
    let map = set.into_iter().enumerate().map(|(i, n)| (n, i)).collect::<BTreeMap<_, _>>();
    let mut ret = vec![];
    for n in vec {
        ret.push(map[n]);
    }
    ret
}

fn power_by<T, F: Fn(&T, &T) -> T>(mut a: T, mut n: usize, e: T, f: F) -> T {
    let mut ans = e;
    while n > 0 {
        if n % 2 != 0 { ans = f(&ans, &a) }
        a = f(&a, &a);
        n /= 2;
    }
    ans
}
